#[cfg(test)]
mod tests {
    use crate::treasury_governance::*;

    #[ink::test]
    fn new_works() {
        let contract = TreasuryGovernance::new();
        assert_eq!(contract.get_total_voters(), 0);
        assert_eq!(contract.get_all_proposal_ids().len(), 0);
    }

    #[ink::test]
    fn voter_registration_works() {
        let mut contract = TreasuryGovernance::new();
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        assert!(contract.register_voter().is_ok());
        assert_eq!(contract.get_total_voters(), 1);
        assert!(contract.is_registered_voter(accounts.alice));
    }

    #[ink::test]
    fn proposal_creation_works() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter first
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let result = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(contract.get_all_proposal_ids().len(), 1);
    }

    #[ink::test]
    fn voting_works() {
        let mut contract = TreasuryGovernance::new();
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote on proposal
        assert!(contract.vote(proposal_id, 0).is_ok());
        
        // Check vote was recorded
        let vote = contract.get_user_vote(proposal_id, accounts.alice).unwrap();
        assert_eq!(vote.choice.option_index, 0);
        assert_eq!(vote.choice.option_text, "Yes");
    }

    #[ink::test]
    fn double_voting_prevention() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote once
        assert!(contract.vote(proposal_id, 0).is_ok());
        
        // Try to vote again - should fail
        assert_eq!(contract.vote(proposal_id, 1), Err(Error::AlreadyVoted));
    }

    #[ink::test]
    fn invalid_voting_options() {
        let mut contract = TreasuryGovernance::new();
        
        // Register voter
        contract.register_voter().unwrap();

        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        // Test empty voting options
        let empty_options = VotingOptions {
            options: vec![],
        };

        let result = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params.clone(),
            empty_options,
        );

        assert_eq!(result, Err(Error::InvalidVotingOptions));

        // Test too many voting options
        let too_many_options = VotingOptions {
            options: (0..11).map(|i| format!("Option {}", i)).collect(),
        };

        let result = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            too_many_options,
        );

        assert_eq!(result, Err(Error::InvalidVotingOptions));
    }

    #[ink::test]
    fn quorum_calculation() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register multiple voters
        contract.register_voter().unwrap();
        
        // Create proposal with 10% quorum
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote on proposal
        assert!(contract.vote(proposal_id, 0).is_ok());
        
        // Check quorum status
        let quorum_reached = contract.has_reached_quorum(proposal_id).unwrap();
        assert!(quorum_reached); // 1 vote out of 1 voter = 100% > 10%
    }

    #[ink::test]
    fn proposal_status_update() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::ThreeDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::Immediately,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote on proposal
        assert!(contract.vote(proposal_id, 0).is_ok());
        
        // Test that update_proposal_status works (even if voting period hasn't ended)
        // This tests the function doesn't crash and handles the case properly
        assert!(contract.update_proposal_status(proposal_id).is_ok());
        
        // Check that proposal is still active (since voting period hasn't ended)
        let proposal = contract.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[ink::test]
    fn proposal_execution() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::ThreeDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::Immediately,
        };

        let voting_options = VotingOptions {
            options: vec!["Yes".to_string(), "No".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote on proposal
        assert!(contract.vote(proposal_id, 0).is_ok());
        
        // Test that execute_proposal fails when proposal is not passed
        // This tests the error handling
        assert_eq!(contract.execute_proposal(proposal_id), Err(Error::ProposalNotReadyForExecution));
        
        // Check proposal status is still active
        let proposal = contract.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[ink::test]
    fn proposal_results_and_statistics() {
        let mut contract = TreasuryGovernance::new();
        let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
        // Register voter
        contract.register_voter().unwrap();

        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };

        let voting_options = VotingOptions {
            options: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()],
        };

        let proposal_id = contract.create_proposal(
            "Multi-option Proposal".to_string(),
            "Test Description".to_string(),
            ProposalType::Governance,
            governance_params,
            voting_options,
        ).unwrap();

        // Vote on proposal
        assert!(contract.vote(proposal_id, 1).is_ok()); // Vote for Option B
        
        // Get proposal results
        let results = contract.get_proposal_results(proposal_id).unwrap();
        assert_eq!(results.proposal_id, proposal_id);
        assert_eq!(results.vote_counts, vec![0, 1, 0]); // Only Option B has 1 vote
        assert_eq!(results.total_votes, 1);
        assert!(results.quorum_reached);
        assert!(results.winning_option.is_some());
        
        let winning = results.winning_option.unwrap();
        assert_eq!(winning.0, 1); // Option B index
        assert_eq!(winning.1, "Option B");
        assert_eq!(winning.2, 1); // 1 vote
        
        // Get detailed results
        let detailed_results = contract.get_detailed_results(proposal_id).unwrap();
        assert_eq!(detailed_results.len(), 3);
        assert_eq!(detailed_results[0], ("Option A".to_string(), 0));
        assert_eq!(detailed_results[1], ("Option B".to_string(), 1));
        assert_eq!(detailed_results[2], ("Option C".to_string(), 0));
        
        // Get winning option
        let winning_option = contract.get_winning_option(proposal_id).unwrap();
        assert!(winning_option.is_some());
        let (option_text, votes) = winning_option.unwrap();
        assert_eq!(option_text, "Option B");
        assert_eq!(votes, 1);
        
        // Get contract statistics
        let stats = contract.get_stats();
        assert_eq!(stats.total_proposals, 1);
        assert_eq!(stats.active_proposals, 1);
        assert_eq!(stats.executed_proposals, 0);
        assert_eq!(stats.total_voters, 1);
    }
}


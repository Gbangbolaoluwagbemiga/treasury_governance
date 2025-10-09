#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod test;

#[ink::contract]
mod treasury_governance {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::primitives::H160;

    /// Proposal Types
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum ProposalType {
        Treasury,
        Governance,
        Technical,
        Other,
    }

    /// Voting Periods
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VotingPeriod {
        ThreeDays,
        SevenDays,
        FourteenDays,
        ThirtyDays,
    }

    impl VotingPeriod {
        /// Convert voting period to block numbers
        pub fn to_blocks(&self) -> u32 {
            match self {
                VotingPeriod::ThreeDays => 3 * 24 * 60 * 10, 
                VotingPeriod::SevenDays => 7 * 24 * 60 * 10,
                VotingPeriod::FourteenDays => 14 * 24 * 60 * 10,
                VotingPeriod::ThirtyDays => 30 * 24 * 60 * 10,
            }
        }
    }

    /// Quorum Thresholds
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum QuorumThreshold {
        Five,
        Ten,
        Twenty,
        TwentyFive,
    }

    impl QuorumThreshold {
        /// Convert quorum threshold to percentage value
        pub fn to_percentage(&self) -> u32 {
            match self {
                QuorumThreshold::Five => 5,
                QuorumThreshold::Ten => 10,
                QuorumThreshold::Twenty => 20,
                QuorumThreshold::TwentyFive => 25,
            }
        }
    }

    /// Execution Delays
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum ExecutionDelay {
        Immediately,
        OneDay,
        TwoDays,
        SevenDays,
    }

    impl ExecutionDelay {
        /// Convert execution delay to block numbers
        pub fn to_blocks(&self) -> u32 {
            match self {
                ExecutionDelay::Immediately => 0,
                ExecutionDelay::OneDay => 24 * 60 * 10, // 1 day * 24 hours * 60 minutes * 10 blocks per minute
                ExecutionDelay::TwoDays => 2 * 24 * 60 * 10,
                ExecutionDelay::SevenDays => 7 * 24 * 60 * 10,
            }
        }
    }

    /// Governance Parameters
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct GovernanceParameters {
        pub voting_period: VotingPeriod,
        pub quorum_threshold: QuorumThreshold,
        pub execution_delay: ExecutionDelay,
    }

    /// Voting Options
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct VotingOptions {
        pub options: Vec<String>,
    }

    /// Vote Choice
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct VoteChoice {
        pub option_index: u32,
        pub option_text: String,
    }

    /// Proposal Status
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
        Executed,
        Expired,
    }

    /// Main Proposal Structure
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Proposal {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub proposal_type: ProposalType,
        pub governance_params: GovernanceParameters,
        pub voting_options: VotingOptions,
        pub proposer: H160,
        pub created_at: u32,
        pub voting_end: u32,
        pub execution_time: u32,
        pub status: ProposalStatus,
        pub vote_counts: Vec<u128>,
        pub total_voters: u32,
    }

    /// Vote Record
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Vote {
        pub voter: H160,
        pub choice: VoteChoice,
        pub timestamp: u32,
        pub weight: u128,
    }

    /// Contract Statistics
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    pub struct ContractStats {
        pub total_proposals: u32,
        pub active_proposals: u32,
        pub executed_proposals: u32,
        pub total_voters: u32,
    }

    /// Proposal Results
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    pub struct ProposalResults {
        pub proposal_id: u32,
        pub vote_counts: Vec<u128>,
        pub total_votes: u128,
        pub quorum_required: u128,
        pub quorum_reached: bool,
        pub winning_option: Option<(u32, String, u128)>,
    }

    /// Custom Error Types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
    pub enum Error {
        ProposalNotFound,
        ProposalNotActive,
        VotingPeriodEnded,
        AlreadyVoted,
        NotAuthorized,
        ProposalNotReadyForExecution,
        InvalidProposal,
        InvalidVotingOptions,
        InvalidOptionIndex,
        QuorumNotReached,
        ExecutionDelayNotMet,
    }

    pub type Result<T> = core::result::Result<T, Error>;



    /// Main Contract Storage
    #[ink(storage)]
    pub struct TreasuryGovernance {
        /// Next proposal ID
        next_proposal_id: u32,
        /// All proposals
        proposals: Mapping<u32, Proposal>,
        /// User votes on proposals (proposal_id -> voter -> Vote)
        votes: Mapping<(u32, H160), Vote>,
        /// List of all proposal IDs
        proposal_ids: Vec<u32>,
        /// Total number of registered voters (for quorum calculation)
        total_voters: u32,
        /// Contract owner
        owner: H160,
        /// Registered voters
        registered_voters: Mapping<H160, bool>,
    }

    impl TreasuryGovernance {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_proposal_id: 1,
                proposals: Mapping::new(),
                votes: Mapping::new(),
                proposal_ids: Vec::new(),
                total_voters: 0,
                owner: Self::env().caller(),
                registered_voters: Mapping::new(),
            }
        }

        /// Register as a voter
        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            if self.registered_voters.get(caller).is_some() {
                return Err(Error::AlreadyVoted); // Reusing error for already registered
            }

            self.registered_voters.insert(caller, &true);
            self.total_voters = self.total_voters.saturating_add(1);

            // self.env().emit_event(VoterRegistered { voter: caller });
            Ok(())
        }

        /// Create a new proposal
        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            description: String,
            proposal_type: ProposalType,
            governance_params: GovernanceParameters,
            voting_options: VotingOptions,
        ) -> Result<u32> {
            // Validate voting options
            if voting_options.options.is_empty() || voting_options.options.len() > 10 {
                return Err(Error::InvalidVotingOptions);
            }

            let current_block = self.env().block_number();
            let voting_period_blocks = governance_params.voting_period.to_blocks();
            let execution_delay_blocks = governance_params.execution_delay.to_blocks();

            // Calculate times with overflow protection
            let voting_end = current_block.saturating_add(voting_period_blocks);
            let execution_time = voting_end.saturating_add(execution_delay_blocks);

            // Initialize vote counts
            let mut vote_counts = Vec::new();
            for _ in 0..voting_options.options.len() {
                vote_counts.push(0u128);
            }

            let proposal = Proposal {
                id: self.next_proposal_id,
                title: title.clone(),
                description,
                proposal_type,
                governance_params,
                voting_options: voting_options.clone(),
                proposer: self.env().caller(),
                created_at: current_block,
                voting_end,
                execution_time,
                status: ProposalStatus::Active,
                vote_counts,
                total_voters: 0,
            };

            // Store proposal
            self.proposals.insert(self.next_proposal_id, &proposal);
            self.proposal_ids.push(self.next_proposal_id);

            let proposal_id = self.next_proposal_id;
            self.next_proposal_id = self.next_proposal_id.saturating_add(1);

            

            Ok(proposal_id)
        }

        /// Vote on a proposal
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32, option_index: u32) -> Result<()> {
            let caller = self.env().caller();
            let current_block = self.env().block_number();

            // Check if voter is registered
            if self.registered_voters.get(caller).is_none() {
                return Err(Error::NotAuthorized);
            }

            // Get proposal
            let mut proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Check if proposal is active
            if proposal.status != ProposalStatus::Active {
                return Err(Error::ProposalNotActive);
            }

            // Check if voting period has ended
            if current_block > proposal.voting_end {
                return Err(Error::VotingPeriodEnded);
            }

            // Check if user has already voted
            if self.votes.get((proposal_id, caller)).is_some() {
                return Err(Error::AlreadyVoted);
            }

            // Validate option index
            if option_index as usize >= proposal.voting_options.options.len() {
                return Err(Error::InvalidOptionIndex);
            }

            // Create vote record
            let vote = Vote {
                voter: caller,
                choice: VoteChoice {
                    option_index,
                    option_text: proposal.voting_options.options[option_index as usize].clone(),
                },
                timestamp: current_block,
                weight: 1, // Simple 1:1 voting weight
            };

            // Store vote
            self.votes.insert((proposal_id, caller), &vote);

            // Update vote counts with overflow protection
            let option_idx = option_index as usize;
            if option_idx < proposal.vote_counts.len() {
                proposal.vote_counts[option_idx] = proposal.vote_counts[option_idx].saturating_add(1);
            }
            proposal.total_voters = proposal.total_voters.saturating_add(1);

            // Update proposal
            self.proposals.insert(proposal_id, &proposal);

    

            Ok(())
        }

        /// Update proposal status (can be called by anyone)
        #[ink(message)]
        pub fn update_proposal_status(&mut self, proposal_id: u32) -> Result<()> {
            let current_block = self.env().block_number();
            let mut proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Only update if proposal is still active
            if proposal.status != ProposalStatus::Active {
                return Ok(());
            }

            // Check if voting period has ended
            if current_block <= proposal.voting_end {
                return Ok(());
            }

            // Calculate quorum with overflow protection
            let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
            let quorum_required = (self.total_voters as u128)
                .saturating_mul(quorum_percentage as u128)
                .saturating_div(100);
            let total_votes: u128 = proposal.vote_counts.iter().sum();

            // Check if quorum is reached
            if total_votes < quorum_required {
                proposal.status = ProposalStatus::Rejected;
                self.proposals.insert(proposal_id, &proposal);
                return Ok(());
            }

            // Find winning option
            let mut max_votes = 0u128;
            let mut _winning_index = 0u32;
            let mut tie = false;

            for (index, &votes) in proposal.vote_counts.iter().enumerate() {
                if votes > max_votes {
                    max_votes = votes;
                    _winning_index = u32::try_from(index).unwrap_or(0);
                    tie = false;
                } else if votes == max_votes && votes > 0 {
                    tie = true;
                }
            }

            // Handle ties
            if tie {
                proposal.status = ProposalStatus::Rejected;
            } else {
                proposal.status = ProposalStatus::Passed;
            }

            self.proposals.insert(proposal_id, &proposal);
            Ok(())
        }

        /// Execute a passed proposal
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<()> {
            let current_block = self.env().block_number();
            let mut proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Check if proposal is passed
            if proposal.status != ProposalStatus::Passed {
                return Err(Error::ProposalNotReadyForExecution);
            }

            // Check if execution delay has passed
            if current_block < proposal.execution_time {
                return Err(Error::ExecutionDelayNotMet);
            }

            // Update status to executed
            proposal.status = ProposalStatus::Executed;
            self.proposals.insert(proposal_id, &proposal);

            Ok(())
        }

        /// Get a specific proposal
        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Result<Proposal> {
            self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)
        }

        /// Get all proposal IDs
        #[ink(message)]
        pub fn get_all_proposal_ids(&self) -> Vec<u32> {
            self.proposal_ids.clone()
        }

        /// Get user's vote on a proposal
        #[ink(message)]
        pub fn get_user_vote(&self, proposal_id: u32, user: H160) -> Result<Vote> {
            self.votes.get((proposal_id, user))
                .ok_or(Error::ProposalNotFound)
        }

        /// Get contract statistics
        #[ink(message)]
        pub fn get_stats(&self) -> ContractStats {
            let mut active_proposals: u32 = 0;
            let mut executed_proposals: u32 = 0;

            for &proposal_id in &self.proposal_ids {
                if let Some(proposal) = self.proposals.get(proposal_id) {
                    match proposal.status {
                        ProposalStatus::Active => active_proposals = active_proposals.saturating_add(1),
                        ProposalStatus::Executed => executed_proposals = executed_proposals.saturating_add(1),
                        _ => {}
                    }
                }
            }

            ContractStats {
                total_proposals: u32::try_from(self.proposal_ids.len()).unwrap_or(0),
                active_proposals,
                executed_proposals,
                total_voters: self.total_voters,
            }
        }

        /// Get total registered voters
        #[ink(message)]
        pub fn get_total_voters(&self) -> u32 {
            self.total_voters
        }

        /// Check if proposal has reached quorum
        #[ink(message)]
        pub fn has_reached_quorum(&self, proposal_id: u32) -> Result<bool> {
            let proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
            let quorum_required = (self.total_voters as u128)
                .saturating_mul(quorum_percentage as u128)
                .saturating_div(100);
            let total_votes: u128 = proposal.vote_counts.iter().sum();

            Ok(total_votes >= quorum_required)
        }

        /// Get proposal results
        #[ink(message)]
        pub fn get_proposal_results(&self, proposal_id: u32) -> Result<ProposalResults> {
            let proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
            let quorum_required = (self.total_voters as u128)
                .saturating_mul(quorum_percentage as u128)
                .saturating_div(100);
            let total_votes: u128 = proposal.vote_counts.iter().sum();
            let quorum_reached = total_votes >= quorum_required;

            // Find winning option
            let mut max_votes = 0u128;
            let mut winning_index = 0u32;
            let mut winning_option = None;

            for (index, &votes) in proposal.vote_counts.iter().enumerate() {
                if votes > max_votes {
                    max_votes = votes;
                    winning_index = u32::try_from(index).unwrap_or(0);
                }
            }

            if max_votes > 0 {
                winning_option = Some((
                    winning_index,
                    proposal.voting_options.options[winning_index as usize].clone(),
                    max_votes,
                ));
            }

            Ok(ProposalResults {
                proposal_id,
                vote_counts: proposal.vote_counts.clone(),
                total_votes,
                quorum_required,
                quorum_reached,
                winning_option,
            })
        }

        /// Get voting options for a proposal
        #[ink(message)]
        pub fn get_voting_options(&self, proposal_id: u32) -> Result<VotingOptions> {
            let proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;
            Ok(proposal.voting_options.clone())
        }

        /// Get detailed results with option names
        #[ink(message)]
        pub fn get_detailed_results(&self, proposal_id: u32) -> Result<Vec<(String, u128)>> {
            let proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            let mut results = Vec::new();
            for (index, &votes) in proposal.vote_counts.iter().enumerate() {
                results.push((
                    proposal.voting_options.options[index].clone(),
                    votes,
                ));
            }

            Ok(results)
        }

        /// Get the winning option and vote count
        #[ink(message)]
        pub fn get_winning_option(&self, proposal_id: u32) -> Result<Option<(String, u128)>> {
            let proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            let mut max_votes = 0u128;
            let mut winning_index = 0usize;

            for (index, &votes) in proposal.vote_counts.iter().enumerate() {
                if votes > max_votes {
                    max_votes = votes;
                    winning_index = index;
                }
            }

            if max_votes > 0 {
                Ok(Some((
                    proposal.voting_options.options[winning_index].clone(),
                    max_votes,
                )))
            } else {
                Ok(None)
            }
        }

        /// Check if an account is a registered voter
        #[ink(message)]
        pub fn is_registered_voter(&self, account: H160) -> bool {
            self.registered_voters.get(account).is_some()
        }
    }

    // Add Default implementation
    impl Default for TreasuryGovernance {
        fn default() -> Self {
            Self::new()
        }
    }
}
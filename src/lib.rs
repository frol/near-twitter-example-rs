// ================================================================================================
// NEAR SMART CONTRACT: Twitter-like Social Media Platform
// ================================================================================================
//
// This smart contract implements a Twitter-like platform on the NEAR blockchain.
// For backend engineers new to blockchain, think of this as a REST API service that:
// - Stores data permanently on a distributed database (the blockchain)
// - Has no central server - it runs on thousands of nodes worldwide
// - Charges small fees (gas) for write operations to prevent spam
// - Is immutable once deployed (like deploying code that can't be changed)
//
// Key NEAR/Blockchain Concepts for Backend Engineers:
// - Smart Contract = Your backend service logic that runs on blockchain
// - Account ID = User identifier (like username, but globally unique)
// - Gas = Computational cost (like AWS Lambda pricing, but for blockchain operations)
// - Storage = Persistent data (like a database, but decentralized and permanent)
// - View Methods = Read operations (free, like GET requests)
// - Call Methods = Write operations (cost gas, like POST/PUT/DELETE requests)

// Import NEAR SDK components - think of this as importing your web framework
use near_sdk::store::IterableMap; // Like HashMap but optimized for blockchain storage
use near_sdk::{env, near, AccountId, PanicOnDefault, Timestamp};

// ================================================================================================
// DATA STRUCTURES
// ================================================================================================

// Tweet represents a single tweet in our social media platform
// The #[near] attribute automatically handles serialization/deserialization
// Think of this as your API response/request DTOs, but for blockchain
#[near(serializers = [borsh, json])]
#[derive(Clone, Debug, PartialEq)]
pub struct Tweet {
    // Unique identifier for this tweet (like auto-increment ID in SQL)
    pub id: u64,

    // NEAR account that created this tweet (like user_id in traditional apps)
    // AccountId is NEAR's version of a username/user identifier
    pub author: AccountId,

    // The actual tweet content (like message body)
    pub text: String,

    // When this tweet was created (NEAR provides nanoseconds since Unix epoch)
    // Think of this as created_at timestamp in your database
    pub timestamp: Timestamp,

    // Number of likes this tweet has received (like a counter field)
    pub likes: u64,
}

// ================================================================================================
// SMART CONTRACT STATE
// ================================================================================================

// TwitterContract is the main contract state - think of this as your application's
// in-memory state that persists between requests (like instance variables in a service class)
//
// In traditional backend:
// - You'd have a database to store tweets
// - You'd have application state in memory/cache
//
// In NEAR:
// - Contract state IS your database (stored on blockchain)
// - State persists between function calls
// - State is automatically loaded/saved by NEAR runtime
#[near(contract_state)]
#[derive(PanicOnDefault)] // Prevents accidental initialization without proper setup
pub struct TwitterContract {
    // Storage for all tweets - like your main tweets table
    // IterableMap is NEAR's version of HashMap optimized for blockchain storage
    // Key: tweet_id, Value: Tweet object
    tweets: IterableMap<u64, Tweet>,

    // Counter for generating unique tweet IDs (like auto-increment in SQL)
    // This ensures each tweet gets a unique identifier
    next_tweet_id: u64,
}

// ================================================================================================
// CONTRACT IMPLEMENTATION
// ================================================================================================

// The #[near] attribute marks this implementation block as containing contract methods
// Think of these as your REST API endpoints, but they run on the blockchain
#[near]
impl TwitterContract {
    // ============================================================================================
    // INITIALIZATION METHOD
    // ============================================================================================

    // Contract constructor - called once when the contract is first deployed
    // Similar to database migrations or initial setup in traditional backends
    // The #[init] attribute marks this as the initialization method
    #[init]
    pub fn new() -> Self {
        Self {
            // Initialize tweet storage with a unique storage prefix
            // "b't'" is a byte string prefix to avoid storage conflicts
            // Think of this as creating a table in your database
            tweets: IterableMap::new(b"t"),

            // Start tweet IDs from 0
            next_tweet_id: 0,
        }
    }

    // ============================================================================================
    // WRITE METHODS (Cost gas, modify state)
    // ============================================================================================

    // Post a new tweet - equivalent to POST /tweets endpoint
    // This is a "call" method that modifies state and costs gas
    pub fn post_tweet(&mut self, text: String) -> Tweet {
        // Get the account that called this method (like extracting user from JWT token)
        // env::predecessor_account_id() returns who made the transaction
        let author = env::predecessor_account_id();

        // Get current blockchain timestamp (like System.currentTimeMillis() in Java)
        // NEAR provides nanoseconds since Unix epoch
        let timestamp = env::block_timestamp();

        // Generate unique ID for this tweet (like auto-increment primary key)
        let tweet_id = self.next_tweet_id;

        // Create the tweet object (like building your entity/model)
        let new_tweet = Tweet {
            id: tweet_id,
            author: author.clone(),
            text,
            timestamp,
            likes: 0, // New tweets start with 0 likes
        };

        // Store the tweet in our "database" (contract storage)
        // This is like INSERT INTO tweets (...) VALUES (...)
        self.tweets.insert(tweet_id, new_tweet.clone());

        // Increment ID counter for next tweet (like auto-increment)
        self.next_tweet_id += 1;

        // Log the action - similar to application logging
        // These logs are stored on blockchain and can be queried
        env::log_str(&format!(
            "Tweet #{} posted by @{} at {}",
            tweet_id, author, timestamp
        ));

        // Return the created tweet (like returning the entity in REST API)
        new_tweet
    }

    // Like a tweet - equivalent to POST /tweets/{id}/like endpoint
    // This modifies state (increments like counter) so it costs gas
    pub fn like_tweet(&mut self, tweet_id: u64) -> Option<Tweet> {
        // Try to get a mutable reference to the tweet
        // This is like: SELECT * FROM tweets WHERE id = ? FOR UPDATE
        if let Some(tweet) = self.tweets.get_mut(&tweet_id) {
            // Increment the like counter (like UPDATE tweets SET likes = likes + 1)
            tweet.likes += 1;

            // Log the like action for transparency/debugging
            env::log_str(&format!(
                "Tweet #{} liked by @{}. Total likes: {}",
                tweet_id,
                env::predecessor_account_id(), // Who liked the tweet
                tweet.likes
            ));

            // Return the updated tweet (clone because we need to return owned data)
            Some(tweet.clone())
        } else {
            // Tweet doesn't exist - log the attempt
            // In REST API, this would be a 404 Not Found
            env::log_str(&format!(
                "Attempt to like non-existent tweet #{} by @{}",
                tweet_id,
                env::predecessor_account_id()
            ));
            None
        }
    }

    // Delete a tweet - equivalent to DELETE /tweets/{id} endpoint
    // Only the tweet author can delete their own tweets (authorization check)
    pub fn delete_tweet(&mut self, tweet_id: u64) {
        // Get who's trying to delete the tweet (like checking JWT/session)
        let caller = env::predecessor_account_id();

        // Check if tweet exists and verify ownership
        // This is like: SELECT author FROM tweets WHERE id = ?
        if let Some(tweet) = self.tweets.get(&tweet_id) {
            // Authorization check - only author can delete their tweet
            // Similar to checking if user owns the resource in REST API
            if tweet.author == caller {
                // Delete the tweet from storage
                // Like: DELETE FROM tweets WHERE id = ?
                self.tweets.remove(&tweet_id);
                env::log_str(&format!("Tweet #{} deleted by @{}", tweet_id, caller));
            } else {
                // Unauthorized deletion attempt - log security event
                // In REST API, this would be 403 Forbidden
                env::log_str(&format!(
                    "User @{} attempted to delete tweet #{} but is not the author.",
                    caller, tweet_id
                ));
            }
        } else {
            // Tweet doesn't exist - log the attempt
            // In REST API, this would be 404 Not Found
            env::log_str(&format!(
                "Attempt to delete non-existent tweet #{} by @{}",
                tweet_id, caller
            ));
        }
    }

    // ============================================================================================
    // READ METHODS (Free, don't modify state)
    // ============================================================================================
    // These are "view" methods - they don't cost gas and don't modify contract state
    // Think of these as GET endpoints in your REST API

    // Get all tweets with pagination - like GET /tweets?offset=0&limit=10
    // from_index: starting position (like OFFSET in SQL)
    // limit: maximum number of tweets to return (like LIMIT in SQL)
    pub fn get_all_tweets(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Tweet> {
        // Set default values if not provided (common REST API pattern)
        let start = from_index.unwrap_or(0);
        let limit_val = limit.unwrap_or(10);

        // Query tweets with pagination (like SELECT * FROM tweets LIMIT x OFFSET y)
        self.tweets
            .iter() // Iterate over all tweets
            .skip(start as usize) // Skip 'start' number of tweets (OFFSET)
            .take(limit_val as usize) // Take only 'limit_val' tweets (LIMIT)
            .map(|(_key, tweet)| tweet.clone()) // Extract tweet objects (ignore keys)
            .collect() // Collect into Vector to return
    }

    // Get specific tweet by ID - like GET /tweets/{id}
    pub fn get_tweet_by_id(&self, tweet_id: u64) -> Option<Tweet> {
        // Simple lookup by primary key
        // Like: SELECT * FROM tweets WHERE id = ?
        self.tweets.get(&tweet_id).cloned()
    }

    // Get tweets by specific author with pagination - like GET /users/{id}/tweets
    // This demonstrates filtering in blockchain storage (no SQL WHERE clause available)
    pub fn get_tweets_by_author(
        &self,
        author_id: AccountId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<Tweet> {
        let start = from_index.unwrap_or(0);
        let limit_val = limit.unwrap_or(10);

        // We need to manually filter since blockchain storage doesn't have SQL-like queries
        // This is like doing: SELECT * FROM tweets WHERE author = ? LIMIT x OFFSET y
        // But we have to iterate through all tweets and filter manually
        let mut author_tweets = Vec::new();
        let mut count = 0;
        let mut current_index = 0;

        // Iterate through all tweets to find matches
        for (_id, tweet) in self.tweets.iter() {
            // Check if this tweet belongs to the requested author
            if tweet.author == author_id {
                // Apply pagination logic
                if current_index >= start && count < limit_val {
                    author_tweets.push(tweet.clone());
                    count += 1;
                }
                current_index += 1;
            }

            // Optimization: stop early if we've found enough tweets
            if count >= limit_val && current_index > start {
                break;
            }
        }

        author_tweets
    }
}

// ================================================================================================
// KEY DIFFERENCES FROM TRADITIONAL BACKEND:
// ================================================================================================
//
// 1. STATE PERSISTENCE:
//    - Traditional: Use database, cache, session storage
//    - NEAR: Contract state IS your database, automatically persisted
//
// 2. USER AUTHENTICATION:
//    - Traditional: JWT tokens, sessions, OAuth
//    - NEAR: Cryptographic signatures, account-based auth built-in
//
// 3. SCALABILITY:
//    - Traditional: Horizontal scaling, load balancers, microservices
//    - NEAR: Automatic scaling across blockchain network
//
// 4. COSTS:
//    - Traditional: Server costs, database costs, bandwidth
//    - NEAR: Gas fees for computations, storage costs per byte
//
// 5. DEPLOYMENT:
//    - Traditional: CI/CD pipelines, blue-green deployments
//    - NEAR: Deploy once, immutable (or upgradeable with special patterns)
//
// 6. DATA CONSISTENCY:
//    - Traditional: ACID transactions, eventual consistency
//    - NEAR: Atomic transactions guaranteed by blockchain consensus
//
// 7. QUERYING DATA:
//    - Traditional: SQL, NoSQL query languages
//    - NEAR: Manual iteration, no complex queries (design for simple access patterns)

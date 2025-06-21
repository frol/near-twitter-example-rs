// ================================================================================================
// SMART CONTRACT TESTING: NEAR Blockchain Testing Guide
// ================================================================================================
//
// This file demonstrates how to test smart contracts on NEAR blockchain.
// For backend engineers new to blockchain, testing smart contracts is similar to
// testing REST APIs, but with some key differences:
//
// 1. ENVIRONMENT SIMULATION:
//    - Traditional: Mock databases, HTTP clients, external services
//    - NEAR: Mock blockchain environment, account simulation, gas tracking
//
// 2. STATE MANAGEMENT:
//    - Traditional: Database transactions, rollbacks between tests
//    - NEAR: Fresh contract instance for each test (like in-memory database)
//
// 3. USER CONTEXT:
//    - Traditional: Mock user sessions, JWT tokens
//    - NEAR: Simulate different blockchain accounts calling methods
//
// 4. TESTING SCOPE:
//    - Traditional: Unit tests (methods), Integration tests (HTTP endpoints)
//    - NEAR: Unit tests (contract methods), Integration tests (cross-contract calls)

// Import NEAR testing utilities and our contract
use near_sdk::{
    test_utils::{accounts, VMContextBuilder}, // Utilities for creating test accounts and context
    testing_env,
    AccountId, // Environment setup and account types
};
use near_twitter_example_rs::TwitterContract; // Our smart contract to test

// ================================================================================================
// TEST MODULE
// ================================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================================
    // TEST UTILITY FUNCTIONS
    // ============================================================================================

    /// Creates a mock blockchain context for testing
    /// Think of this as setting up a mock HTTP request with user authentication
    ///
    /// In traditional backend testing, you might do:
    /// ```
    /// MockHttpServletRequest request = new MockHttpServletRequest();
    /// request.addHeader("Authorization", "Bearer " + jwtToken);
    /// ```
    ///
    /// In NEAR testing, we set up blockchain context:
    /// - Who is calling the contract (predecessor_account_id)
    /// - What account the contract is deployed to (current_account_id)
    /// - Who signed the transaction (signer_account_id)
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            // Set the contract's account (like setting the server hostname)
            .current_account_id(accounts(0))
            // Set who signed the transaction (usually same as predecessor)
            .signer_account_id(predecessor_account_id.clone())
            // Set who is calling the contract (like user ID from JWT)
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // ============================================================================================
    // INITIALIZATION TESTS
    // ============================================================================================

    /// Test contract initialization (constructor)
    /// Similar to testing that your service starts up correctly
    #[test]
    fn test_new() {
        // Setup: Create a mock blockchain environment
        // This is like setting up a test database and mock HTTP context
        let context = get_context(accounts(1));
        testing_env!(context.build());

        // Act: Initialize the contract (like calling your service constructor)
        let _contract = TwitterContract::new();

        // Assert: Contract is initialized successfully if no panic occurs
        // In blockchain, panics are like throwing exceptions - they revert all changes
        // If we reach this point, initialization was successful
    }

    // ============================================================================================
    // WRITE OPERATION TESTS (Methods that modify state)
    // ============================================================================================

    /// Test posting a new tweet
    /// Similar to testing POST /tweets endpoint
    #[test]
    fn test_post_tweet() {
        // Setup: Initialize test environment and contract
        let context = get_context(accounts(1)); // accounts(1) = user who will post tweet
        testing_env!(context.build());
        let mut contract = TwitterContract::new();

        // Act: Post a tweet (like making a POST request)
        let tweet = contract.post_tweet("Hello NEAR!".to_string());

        // Assert: Verify the tweet was created correctly
        // Check all the fields like you would verify a REST API response
        assert_eq!(tweet.id, 0); // First tweet should have ID 0
        assert_eq!(tweet.author, accounts(1)); // Author should be the caller
        assert_eq!(tweet.text, "Hello NEAR!"); // Content should match input
        assert_eq!(tweet.likes, 0); // New tweets start with 0 likes
                                    // Note: We can't check timestamp easily in tests, but it's set by the contract
    }

    /// Test liking tweets
    /// Similar to testing POST /tweets/{id}/like endpoint
    #[test]
    fn test_like_tweet() {
        // Setup: Create a tweet to like
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = TwitterContract::new();
        contract.post_tweet("Likeable tweet".to_string());

        // Act & Assert: Like the tweet
        let liked_tweet = contract.like_tweet(0);
        assert!(liked_tweet.is_some()); // Should return the tweet
        assert_eq!(liked_tweet.unwrap().likes, 1); // Should have 1 like

        // Act & Assert: Like the same tweet again (multiple likes allowed)
        let liked_again = contract.like_tweet(0);
        assert!(liked_again.is_some());
        assert_eq!(liked_again.unwrap().likes, 2); // Should have 2 likes

        // Act & Assert: Try to like non-existent tweet (error case)
        let non_existent = contract.like_tweet(999);
        assert!(non_existent.is_none()); // Should return None (like 404)
    }

    /// Test tweet deletion with authorization
    /// Similar to testing DELETE /tweets/{id} with ownership verification
    #[test]
    fn test_delete_tweet() {
        // Setup: Create a tweet from user 1
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = TwitterContract::new();
        contract.post_tweet("Tweet to delete".to_string());

        // Verify tweet exists
        assert!(contract.get_tweet_by_id(0).is_some());

        // Act: Delete the tweet as the author (should succeed)
        contract.delete_tweet(0);

        // Assert: Tweet should be deleted
        assert!(contract.get_tweet_by_id(0).is_none());

        // Edge Case: Try to delete non-existent tweet (should not panic)
        contract.delete_tweet(999); // Should handle gracefully

        // Authorization Test: Create another tweet and try to delete as different user
        contract.post_tweet("Another tweet".to_string());

        // Switch to different user context (like switching JWT token)
        context.predecessor_account_id(accounts(2));
        testing_env!(context.build());

        // Act: Try to delete as different user (should fail)
        contract.delete_tweet(1);

        // Switch back to original author to verify tweet still exists
        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        // Assert: Tweet should still exist (deletion should have failed)
        assert!(contract.get_tweet_by_id(1).is_some());
    }

    // ============================================================================================
    // READ OPERATION TESTS (Methods that don't modify state)
    // ============================================================================================

    /// Test retrieving specific tweet by ID
    /// Similar to testing GET /tweets/{id} endpoint
    #[test]
    fn test_get_tweet_by_id() {
        // Setup: Create a contract and post a tweet
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = TwitterContract::new();
        let posted_tweet = contract.post_tweet("Test tweet".to_string());

        // Act: Retrieve the tweet by ID
        let retrieved_tweet = contract.get_tweet_by_id(0);

        // Assert: Should return the correct tweet
        assert!(retrieved_tweet.is_some());
        assert_eq!(retrieved_tweet.unwrap(), posted_tweet);

        // Edge Case: Try to get non-existent tweet
        let non_existent = contract.get_tweet_by_id(999);
        assert!(non_existent.is_none()); // Should return None (like 404)
    }

    /// Test getting all tweets with pagination
    /// Similar to testing GET /tweets?offset=1&limit=1 endpoint
    #[test]
    fn test_get_all_tweets() {
        // Setup: Create multiple tweets to test pagination
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = TwitterContract::new();

        // Create test data
        contract.post_tweet("First tweet".to_string());
        contract.post_tweet("Second tweet".to_string());
        contract.post_tweet("Third tweet".to_string());

        // Test: Get all tweets (no pagination)
        let all_tweets = contract.get_all_tweets(None, None);
        assert_eq!(all_tweets.len(), 3);
        assert_eq!(all_tweets[0].text, "First tweet");
        assert_eq!(all_tweets[1].text, "Second tweet");
        assert_eq!(all_tweets[2].text, "Third tweet");

        // Test: Pagination - skip first tweet, get only 1 tweet
        // This is like calling GET /tweets?offset=1&limit=1
        let limited_tweets = contract.get_all_tweets(Some(1), Some(1));
        assert_eq!(limited_tweets.len(), 1);
        assert_eq!(limited_tweets[0].text, "Second tweet");
    }

    /// Test getting tweets by specific author
    /// Similar to testing GET /users/{id}/tweets endpoint
    #[test]
    fn test_get_tweets_by_author() {
        // Setup: Create tweets from multiple authors
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = TwitterContract::new();

        // User 1 posts tweets
        contract.post_tweet("Tweet from user 1".to_string());
        contract.post_tweet("Another tweet from user 1".to_string());

        // Switch to user 2 (like logging in as different user)
        context.predecessor_account_id(accounts(2));
        testing_env!(context.build());
        contract.post_tweet("Tweet from user 2".to_string());

        // Test: Get tweets by user 1
        let user1_tweets = contract.get_tweets_by_author(accounts(1), None, None);
        assert_eq!(user1_tweets.len(), 2);
        assert_eq!(user1_tweets[0].author, accounts(1));
        assert_eq!(user1_tweets[1].author, accounts(1));

        // Test: Get tweets by user 2
        let user2_tweets = contract.get_tweets_by_author(accounts(2), None, None);
        assert_eq!(user2_tweets.len(), 1);
        assert_eq!(user2_tweets[0].author, accounts(2));

        // Test: Get tweets by non-existent user (edge case)
        let no_tweets = contract.get_tweets_by_author(accounts(3), None, None);
        assert_eq!(no_tweets.len(), 0);
    }
}

// ================================================================================================
// TESTING BEST PRACTICES FOR BLOCKCHAIN:
// ================================================================================================
//
// 1. ISOLATION:
//    - Each test gets a fresh contract instance (like fresh database)
//    - Tests don't interfere with each other
//
// 2. CONTEXT SIMULATION:
//    - Always set up proper blockchain context before testing
//    - Simulate different users by changing predecessor_account_id
//
// 3. STATE VERIFICATION:
//    - Test both success and failure cases
//    - Verify state changes persist correctly
//    - Test authorization and ownership checks
//
// 4. EDGE CASES:
//    - Non-existent resources (like 404 errors)
//    - Unauthorized operations (like 403 errors)
//    - Invalid inputs (like 400 errors)
//
// 5. GAS CONSIDERATIONS:
//    - In production, test gas consumption
//    - Ensure operations don't exceed gas limits
//    - Unit tests don't track gas, but integration tests can
//
// 6. PAGINATION TESTING:
//    - Test default values
//    - Test boundary conditions
//    - Test offset/limit combinations
//
// 7. MULTI-USER SCENARIOS:
//    - Test interactions between different accounts
//    - Verify authorization works correctly
//    - Test concurrent operations (though unit tests are sequential)

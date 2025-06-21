# NEAR Twitter Example - Smart Contract for Backend Engineers

A comprehensive guide to building Twitter-like social media on [NEAR blockchain](https://dev.near.org), designed for backend engineers transitioning to Web3 development.

## 🎯 What You'll Learn

This project demonstrates how to build a decentralized social media platform using NEAR Protocol. If you're a backend engineer familiar with REST APIs, databases, and server architecture, this guide will help you understand blockchain development through familiar concepts.

## 🔄 Backend vs Blockchain: Key Mappings

| Traditional Backend | NEAR Blockchain | Example |
|-------------------|-----------------|---------|
| REST API Endpoints | Contract Methods | `POST /tweets` → `post_tweet()` |
| Database Tables | Contract State | `tweets` table → `IterableMap<u64, Tweet>` |
| User Authentication | Account-based Auth | JWT token → `env::predecessor_account_id()` |
| Server Deployment | Contract Deployment | Docker deploy → `cargo near deploy` |
| Database Transactions | Blockchain Transactions | SQL transaction → contract call |
| Request Validation | Input Validation | Same patterns, but panics revert |
| Logging | Event Emission | `console.log()` → `env::log_str()` |
| Horizontal Scaling | Network Scaling | Load balancers → blockchain nodes |

## 🏗️ Architecture Overview

```
Traditional 3-Tier Architecture:
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Frontend  │────│   Backend   │────│  Database   │
│   (React)   │    │ (Node/Java) │    │ (PostgreSQL)│
└─────────────┘    └─────────────┘    └─────────────┘

NEAR Blockchain Architecture:
┌─────────────┐    ┌─────────────────────────────────┐
│   Frontend  │────│        NEAR Network             │
│   (React)   │    │  ┌─────────────┐ ┌─────────────┐│
└─────────────┘    │  │   Contract  │ │   Storage   ││
                   │  │   (Logic)   │ │   (State)   ││
                   │  └─────────────┘ └─────────────┘│
                   └─────────────────────────────────┘
```

## 📊 Data Models

### Tweet Structure
```rust
pub struct Tweet {
    pub id: u64,           // Auto-increment ID (like primary key)
    pub author: AccountId, // User identifier (like foreign key to users)
    pub text: String,      // Tweet content (like varchar field)
    pub timestamp: u64,    // Creation time (like created_at)
    pub likes: u64,        // Like counter (like aggregated count)
}
```

**Comparison with SQL:**
```sql
CREATE TABLE tweets (
    id BIGSERIAL PRIMARY KEY,
    author VARCHAR(64) NOT NULL,
    text TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    likes BIGINT DEFAULT 0
);
```

## 🚀 Quick Start

### Prerequisites
```bash
# Install Rust (like installing Node.js/Java)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target (like adding mobile compilation targets)
rustup target add wasm32-unknown-unknown

# Install NEAR CLI tools (like installing AWS CLI)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/cargo-near/releases/latest/download/cargo-near-installer.sh | sh
```

### Development Workflow

#### 1. Build Contract (like `npm run build`)
```bash
cargo near build
```

#### 2. Test Contract (like `npm test`)
```bash
cargo test
```

#### 3. Deploy Contract (like deploying to AWS/Heroku)
```bash
# Create testnet account (like creating staging environment)
cargo near create-dev-account

# Deploy to testnet (like deploying to staging)
cargo near deploy build-non-reproducible-wasm with-init-call new json-args '{}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet

# Deploy to mainnet (like deploying to production)
cargo near deploy build-reproducible-wasm with-init-call new json-args '{}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config mainnet
```

## 📖 API Reference

### Write Methods (Cost Gas - like POST/PUT/DELETE)

#### `post_tweet(text: String) -> Tweet`
Create a new tweet.

**Traditional equivalent:** `POST /tweets`
```javascript
// REST API (with JWT auth)
POST /tweets
Authorization: Bearer <jwt-token>
{
  "text": "Hello World!"
}

// NEAR Contract Call
near contract call-function \
  as-transaction '<your-contract.testnet>' post_tweet \
  json-args '{"text": "Hello World!"}' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as '<some-user.testnet>'
```

#### `like_tweet(tweet_id: u64) -> Option<Tweet>`
Like a specific tweet.

**Traditional equivalent:** `POST /tweets/{id}/like`
```javascript
// REST API (with JWT auth)
POST /tweets/123/like
Authorization: Bearer <jwt-token>

// NEAR Contract Call
near contract call-function \
  as-transaction '<your-contract.testnet>' like_tweet \
  json-args '{"tweet_id": 123}' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as '<some-user.testnet>'
```

#### `delete_tweet(tweet_id: u64)`
Delete a tweet (only by author).

**Traditional equivalent:** `DELETE /tweets/{id}`
```javascript
// REST API (with JWT auth)
DELETE /tweets/123
Authorization: Bearer <jwt-token>

// NEAR Contract Call (with account auth)
near contract call-function \
  as-transaction '<your-contract.testnet>' delete_tweet \
  json-args '{"tweet_id": 123}' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as '<tweet-author.testnet>'
```

### Read Methods (Free - like GET)

#### `get_all_tweets(from_index?: u64, limit?: u64) -> Tweet[]`
Get paginated list of all tweets.

**Traditional equivalent:** `GET /tweets?offset=0&limit=10`
```javascript
// REST API
GET /tweets?offset=0&limit=10

// NEAR Contract View
near contract call-function \
  as-read-only '<your-contract.testnet>' get_all_tweets \
  json-args '{"from_index": 0, "limit": 10}'
```

#### `get_tweet_by_id(tweet_id: u64) -> Option<Tweet>`
Get specific tweet by ID.

**Traditional equivalent:** `GET /tweets/{id}`
```javascript
// REST API
GET /tweets/123

// NEAR Contract View
near contract call-function \
  as-read-only '<your-contract.testnet>' get_tweet_by_id \
  json-args '{"tweet_id": 123}'
```

#### `get_tweets_by_author(author_id: AccountId, from_index?: u64, limit?: u64) -> Tweet[]`
Get tweets by specific author.

**Traditional equivalent:** `GET /users/{id}/tweets`
```javascript
// REST API
GET /users/john/tweets?offset=0&limit=10

// NEAR Contract View
near contract call-function \
  as-read-only '<your-contract.testnet>' get_tweets_by_author \
  json-args '{"author_id": "john.testnet", "from_index": 0, "limit": 10}'
```

## 🧪 Testing Strategy

### Unit Tests (like testing business logic)
```rust
#[test]
fn test_post_tweet() {
    let mut contract = TwitterContract::new();
    let tweet = contract.post_tweet("Hello!".to_string());
    assert_eq!(tweet.text, "Hello!");
}
```

### Integration Tests (like testing API endpoints)
```bash
# Deploy to testnet and test with real blockchain
cargo near deploy
near contract call-function \
    as-transaction '<your-contract.testnet>' post_tweet \
    json-args '{"text": "Integration test"}' \
    prepaid-gas '30.0 Tgas' \
    attached-deposit '0 NEAR' \
    sign-as '<some-user.testnet>'
```

### Performance Testing
```bash
# Test gas consumption (like testing API response times)
cargo test
```

## 🔐 Security Considerations

### Authorization (like JWT/OAuth in traditional backend)
```rust
pub fn delete_tweet(&mut self, tweet_id: u64) {
    let caller = env::predecessor_account_id(); // Get authenticated user
    if let Some(tweet) = self.tweets.get(&tweet_id) {
        if tweet.author == caller { // Check ownership
            self.tweets.remove(&tweet_id);
        } else {
            env::panic_str("Unauthorized"); // Like 403 Forbidden
        }
    }
}
```

### Input Validation (same patterns as traditional backend)
```rust
pub fn post_tweet(&mut self, text: String) -> Tweet {
    // Validate input (like request validation middleware)
    if text.is_empty() {
        near_sdk::env::panic_str("Tweet cannot be empty");
    }
    if text.len() > 280 {
        near_sdk::env::panic_str("Tweet too long");
    }
    // ... rest of implementation
}
```

### Gas Optimization (like performance optimization)
```rust
// Bad: O(n) iteration on every call
pub fn get_like_count(&self) -> u64 {
    self.tweets.values().map(|t| t.likes).sum() // Expensive!
}

// Good: Store aggregated data
pub struct TwitterContract {
    tweets: IterableMap<u64, Tweet>,
    total_likes: u64, // Pre-computed aggregate
}
```

## 📈 Monitoring & Observability

### Logging (like application logs)
```rust
env::log_str(&format!("Tweet {} posted by {}", tweet_id, author));
```

### Metrics (available through NEAR indexers)
- Transaction volume
- Gas consumption
- Active users
- Error rates

### Deployment
```bash
# 1. Final testing on testnet
cargo test
cargo near deploy build-non-reproducible-wasm with-init-call new json-args '{}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet

# 2. Deploy to mainnet
cargo near deploy build-reproducible-wasm with-init-call new json-args '{}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config mainnet

# 3. Verify deployment
near contract call-function as-read-only '<your-contract>.near' get_all_tweets json-args '{}'
```

## 🔄 Migration from Traditional Backend

### Database → Contract State
```rust
// Instead of SQL schema migrations
// Use versioned contract deployments with state migration

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedTwitterContract {
    V1(TwitterContractV1),
    V2(TwitterContract), // New version with additional fields
}
```

### API Versioning → Contract Upgrades
```rust
// Add new methods while keeping old ones for backwards compatibility
pub fn post_tweet_v2(&mut self, text: String, media: Option<String>) -> Tweet {
    // New functionality
}

pub fn post_tweet(&mut self, text: String) -> Tweet {
    self.post_tweet_v2(text, None) // Delegate to new version
}
```

## 📚 Additional Resources

### NEAR Protocol
- [NEAR Documentation](https://docs.near.org) - Official documentation
- [NEAR SDK Rust](https://github.com/near/near-sdk-rs) - Framework documentation
- [NEAR Examples](https://github.com/near/near-sdk-rs/tree/master/examples) - More contract examples

### Blockchain Concepts for Backend Engineers
- [Smart Contracts vs Microservices](https://docs.near.org/concepts/basics/accounts/smartcontract)
- [Gas vs API Rate Limiting](https://docs.near.org/concepts/basics/transactions/gas)
- [Consensus vs Database ACID](https://docs.near.org/concepts/basics/validators)

### Development Tools
- [NEAR CLI](https://github.com/near/near-cli-rs) - Command line interface
- [NEAR Explorer](https://explorer.near.org) - Blockchain explorer (like database admin tools)
- [NEAR Wallet](https://wallet.near.org) - User account management

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for your changes
4. Ensure all tests pass (`cargo test`)
5. Update documentation if needed
6. Submit pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💡 Need Help?

- **NEAR Discord**: [https://near.chat](https://near.chat)
- **Stack Overflow**: Tag questions with `nearprotocol`
- **GitHub Issues**: Report bugs or request features

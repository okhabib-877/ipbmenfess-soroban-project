# IPB Menfess DApp

**IPB Menfess DApp** - Blockchain-Based Decentralized Confession and Feedback System

## Project Description

IPB Menfess DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for managing student complaints and confessions directly on the blockchain. The contract ensures that your data is stored transparently and is only manageable through predefined smart contract functions, eliminating reliance on centralized database providers.

The system allows students to create, view, and delete their menfess, leveraging the efficiency and security of the Stellar network. Each menfess is uniquely identified and stored within the contract's instance storage, ensuring data persistence and reliability.

## Project Vision

Our vision is to revolutionize campus communication in the digital age by:

- **Decentralizing Data**: Moving student feedback from centralized servers to a global, distributed blockchain
- **Ensuring Ownership**: Empowering students to have complete control and ownership over their digital voices and information
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of confessions that cannot be altered or deleted by third parties
- **Enhancing Privacy**: Leveraging blockchain security to protect personal information from unauthorized access
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, not by company promises

We envision a future where digital information is truly personal and sovereign, empowering individuals with complete autonomy over their digital assets.

## Key Features

### 1. **Simple Menfess Creation**

- Create a menfess with just one function call
- Specify NIM and content for each menfess
- Automated ID generation for unique identification
- Persistent storage on the Stellar blockchain

### 2. **Efficient Data Retrieval**

- Fetch all stored menfess in a single call
- Structured data representation for easy frontend integration
- Quick access to the entire campus confession collection
- Real-time synchronization with the blockchain state

### 3. **Secure Deletion**

- Remove specific menfess using their unique IDs
- Permanent removal from the contract storage
- Clean and efficient storage management
- Immediate update of the menfess list after deletion

### 4. **Transparency and Security**

- View all menfess activities on the blockchain
- Blockchain-based verification of all storage actions
- Immutable records of menfess creation and deletion
- Protected against unauthorized modifications

### 5. **Stellar Network Integration**

- Leverages the high speed and low cost of Stellar
- Built using the modern Soroban Smart Contract SDK
- Scalable architecture for growing menfess collections
- Interoperable with other Stellar-based services

## Contract Details

- Contract Address: CCRPRFHIRBN424AUQLGJ7LEVS5MKPETSZMOUBMTFELFDK6OWUDFLRICG
  (Screenshot has been removed)

## Future Scope

### Short-Term Enhancements

1. **Menfess Encryption**: Support for end-to-end encryption of menfess content for enhanced privacy
2. **Category Management**: Add tags and categories to organize confessions efficiently
3. **Rich Text Support**: Extend support beyond plain text to include Markdown and formatted content
4. **Search Functionality**: Implement advanced search filters for large menfess collections

### Medium-Term Development

5. **Collaborative Menfess**: Implement multi-signature requirements for shared or collaborative confessions
   - Shared access for multiple addresses
   - Permission-based editing and viewing
   - Version history tracking
6. **Notification System**: Off-chain bridge to alert users of new updates or trending menfess
7. **Asset Attachment**: Capability to attach digital assets or tokens to specific menfess
8. **Inter-Contract Integration**: Allow other smart contracts to interact with and store data in the menfess contract

### Long-Term Vision

9. **Cross-Chain Synchronization**: Extend menfess storage to multiple blockchain networks
10. **Decentralized UI Hosting**: Host the frontend on IPFS or similar decentralized platforms
11. **AI-Powered Summarization**: Optional integration with AI to help users summarize campus trending topics
12. **Privacy Layers**: Implement zero-knowledge proofs for completely private menfess content
13. **DAO Governance**: Community-driven protocol improvements and feature prioritization
14. **Identity Management**: Integration with decentralized identity (DID) systems for student management

### Enterprise Features

15. **Campus Documentation**: Adapt the system for secure campus record-keeping
16. **Immutable Logging**: Create time-locked logs for audit purposes
17. **Automated Reporting**: Automatic menfess triggers for periodic reporting
18. **Multi-Language Support**: Expand accessibility with internationalization

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

- `tambah_keluhan()` - Create a new menfess with NIM and content
- `get_semua_keluhan()` - Retrieve all stored menfess from the contract
- `hapus_keluhan()` - Remove a specific menfess by its ID

---

**IPB Menfess DApp** - Securing Your Thoughts on the Blockchain
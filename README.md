# Solana Escrow Program

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-1.18.14-blue)](https://solana.com/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A secure, trustless escrow program built on the Solana blockchain that enables atomic swaps between SPL tokens and SOL. This program facilitates peer-to-peer exchanges without requiring a trusted intermediary.

## 🚀 Features

- **Trustless Exchanges**: No intermediary required - smart contracts handle the entire process
- **Atomic Swaps**: Either both parties receive their assets or the transaction is reverted
- **SPL Token Support**: Full support for any SPL token standard
- **Cancellation Support**: Initializers can cancel pending escrows and reclaim their tokens
- **Rent-Exempt Safety**: Ensures all accounts maintain minimum balance requirements
- **Security Audited**: Implements best practices for Solana program development

## 📋 Table of Contents

- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Program Instructions](#program-instructions)
- [Account Structure](#account-structure)
- [Security Considerations](#security-considerations)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## 🏗️ Architecture

The escrow program implements a three-step process:

1. **Initialize Escrow**: The initializer deposits SPL tokens and specifies the amount of SOL they want in return
2. **Exchange**: A counterparty provides the requested SOL and receives the SPL tokens
3. **Cancel**: The initializer can cancel the escrow and reclaim their tokens (if no exchange has occurred)

### Program Flow

```
Initializer                     Counterparty
     │                               │
     ▼                               │
┌─────────────────┐                  │
│ Initialize      │                  │
│ Escrow          │                  │
│                 │                  │
│ • Deposit SPL   │                  │
│ • Set SOL price │                  │
└─────────────────┘                  │
     │                               │
     ▼                               ▼
┌─────────────────────────────────────────┐
│            Escrow Active                │
│                                         │
│  ┌─────────────┐    ┌─────────────┐    │
│  │   Cancel    │    │  Exchange   │    │
│  │   (Init)    │    │  (Counter)  │    │
│  └─────────────┘    └─────────────┘    │
└─────────────────────────────────────────┘
```

## 🛠️ Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.18+
- [Node.js](https://nodejs.org/) (for testing)

### Install Solana CLI Tools

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.14/install)"

# Install Solana BPF toolchain
cargo install solana-platform-tools

# Verify installation
solana --version
cargo build-sbf --version
```

### Build the Program

```bash
# Clone the repository
git clone https://github.com/0x-Professor/Escrow-Program-Solana.git
cd escrow-program

# Build the program
cargo build-sbf

# The compiled program will be in target/sbf-solana-solana/release/
```

## 🎯 Usage

### Deployment

```bash
# Configure Solana CLI for your target cluster
solana config set --url https://api.devnet.solana.com  # or mainnet-beta

# Deploy the program
solana program deploy target/sbf-solana-solana/release/vesting_program.so

# Note the program ID from the deployment output
```

### Client Integration

#### Initialize an Escrow

```javascript
import { PublicKey, Transaction, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

// Initialize escrow instruction
const initEscrowInstruction = {
    keys: [
        { pubkey: initializer.publicKey, isSigner: true, isWritable: true },
        { pubkey: tempTokenAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: initializerTokenAccount.publicKey, isSigner: false, isWritable: false },
        { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: ESCROW_PROGRAM_ID,
    data: Buffer.from([0, ...new BN(expectedAmount).toArray("le", 8)]),
};
```

#### Exchange Tokens

```javascript
// Exchange instruction
const exchangeInstruction = {
    keys: [
        { pubkey: counterparty.publicKey, isSigner: true, isWritable: true },
        { pubkey: counterpartyReceiveTokenAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: pdaDepositTokenAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: initializerMainAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: initializerReceiveTokenAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: PDA_ACCOUNT, isSigner: false, isWritable: false },
    ],
    programId: ESCROW_PROGRAM_ID,
    data: Buffer.from([1, ...new BN(expectedAmount).toArray("le", 8)]),
};
```

## 📝 Program Instructions

### InitEscrow

Initializes a new escrow with SPL tokens.

**Parameters:**
- `amount`: The amount of SOL expected in return (in lamports)

**Accounts:**
1. `[signer, writable]` Initializer account
2. `[writable]` Temporary token account (holds the SPL tokens)
3. `[]` Initializer's main token account
4. `[writable]` Escrow account (stores escrow state)
5. `[]` Rent sysvar
6. `[]` Token program

### Exchange

Completes the token exchange.

**Parameters:**
- `amount`: The amount of SOL to send (must match expected amount)

**Accounts:**
1. `[signer, writable]` Counterparty account
2. `[writable]` Counterparty's token receiving account
3. `[writable]` PDA's temporary token account
4. `[writable]` Initializer's main account (receives SOL)
5. `[writable]` Initializer's token receiving account
6. `[writable]` Escrow account
7. `[]` Token program
8. `[]` PDA account

### Cancel

Cancels the escrow and returns tokens to the initializer.

**Accounts:**
1. `[signer, writable]` Initializer account
2. `[writable]` PDA's temporary token account
3. `[writable]` Initializer's token account
4. `[writable]` Escrow account
5. `[]` Token program
6. `[]` PDA account

## 🗃️ Account Structure

### Escrow Account

```rust
pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub expected_amount: u64,
}
```

**Size**: 73 bytes
- `is_initialized`: 1 byte
- `initializer_pubkey`: 32 bytes
- `temp_token_account_pubkey`: 32 bytes
- `expected_amount`: 8 bytes

## 🔐 Security Considerations

### Implemented Security Measures

1. **Signer Verification**: All sensitive operations require proper signatures
2. **Account Ownership Validation**: Ensures accounts are owned by expected programs
3. **Rent Exemption Checks**: Prevents accounts from being deallocated
4. **Amount Validation**: Verifies token amounts match expectations
5. **Initialization Guards**: Prevents double-initialization of escrow accounts
6. **PDA Authority**: Uses Program Derived Addresses for secure token custody

### Best Practices

- Always validate account ownership
- Check for rent exemption on all accounts
- Use PDAs for program-controlled accounts
- Implement proper error handling
- Validate all numeric inputs to prevent overflow
- Use the latest Solana program dependencies

### Known Limitations

- Only supports SPL tokens to SOL exchanges
- Requires manual account creation by clients
- No built-in price oracles (relies on user-specified amounts)

## 🧪 Development

### Project Structure

```
src/
├── entrypoint.rs      # Program entrypoint
├── error.rs          # Custom error definitions
├── instruction.rs    # Instruction parsing
├── lib.rs           # Library root
├── processor.rs     # Core program logic
└── state.rs         # Account state structures
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests (requires local validator)
cargo test-sbf
```

### Local Development

```bash
# Start local validator
solana-test-validator

# Deploy to local cluster
solana config set --url localhost
solana program deploy target/sbf-solana-solana/release/vesting_program.so
```

## 🔧 Testing

### Unit Tests

The program includes comprehensive unit tests for all major functions:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_process_init_escrow
```

### Integration Testing

For end-to-end testing, use the Solana test framework:

```bash
# Build and test
cargo build-sbf
cargo test-sbf
```

### Test Coverage

Current test coverage includes:
- ✅ Escrow initialization
- ✅ Token exchange
- ✅ Escrow cancellation
- ✅ Error handling
- ✅ Account validation

## 📊 Performance

### Transaction Costs

| Operation | Compute Units | Typical Cost (SOL) |
|-----------|---------------|-------------------|
| InitEscrow | ~15,000 | ~0.000015 |
| Exchange | ~20,000 | ~0.000020 |
| Cancel | ~12,000 | ~0.000012 |

### Limitations

- Maximum escrow amount: `u64::MAX` lamports
- No expiration mechanism (manual cancellation required)
- Single SPL token type per escrow

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

### Quick Start for Contributors

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check our [docs](./docs/) folder
- **Issues**: Report bugs via [GitHub Issues](https://github.com/0x-Professor/Escrow-Program-Solana/issues)
- **Discussions**: Join our [GitHub Discussions](https://github.com/0x-Professor/Escrow-Program-Solana/discussions)

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. Users should conduct thorough testing and auditing before using in production environments. The authors are not responsible for any loss of funds or other damages.

## 🏆 Acknowledgments

- Built on [Solana](https://solana.com/)
- Uses [SPL Token Program](https://spl.solana.com/)
- Inspired by Ethereum escrow patterns
- Community feedback and contributions

---

**Made with ❤️ by [0x-Professor](https://github.com/0x-Professor)**

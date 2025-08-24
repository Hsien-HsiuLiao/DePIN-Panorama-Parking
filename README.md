# 🚗 Panorama Parking

<div align="center">
  <img src="PPlogo.png" alt="Panorama Parking Logo" width="200">
</div>

> **Revolutionizing Parking with Decentralized Infrastructure and Smart Contracts**

[![Solana](https://img.shields.io/badge/Solana-000000?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-000000?style=for-the-badge&logo=anchor&logoColor=white)](https://www.anchor-lang.com/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)
[![Next.js](https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)](https://nextjs.org/)
[![React Native](https://img.shields.io/badge/React_Native-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactnative.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![MongoDB](https://img.shields.io/badge/MongoDB-4EA94B?style=for-the-badge&logo=mongodb&logoColor=white)](https://www.mongodb.com/)

## 📖 Overview

Panorama Parking is a decentralized parking marketplace built on Solana that leverages LoRaWAN sensors and blockchain technology to create an efficient, transparent, and user-friendly parking ecosystem. The platform allows users to list, discover, and reserve parking spaces while providing real-time availability updates through sensor integration.

## ✨ Features

- 🏠 **Smart Parking Listings**: Create and manage parking space listings with detailed information
- 📍 **GPS Integration**: Precise location tracking with latitude/longitude coordinates
- ⏰ **Real-time Availability**: Dynamic availability windows with start/end timestamps
- 🔔 **Multi-channel Notifications**: App, email, and phone notification preferences
- 💰 **Flexible Pricing**: Set custom rental rates for your parking spaces
- 📱 **Cross-platform App**: React Native app with web compatibility
- 🔗 **LoRaWAN Sensor Integration**: Real-time parking status updates via Switchboard feeds
- 🚀 **Solana Blockchain**: Fast, low-cost transactions with high throughput

## 🏗️ Architecture

The project consists of three main components:

### 1. Solana Smart Contracts (`anchor-panorama-parking/`)
- **Program ID**: `FXUQwDsKJNrYFsfiUokPbH4BSrZtoC9m8HpoiMvYxtSE`
- **Network**: Solana Devnet
- **Framework**: Anchor Framework

#### Core Instructions:
- `initialize`: Initialize the marketplace with name and fee structure
- `list`: Create new parking space listings
- `update_listing`: Modify existing listing details
- `delete_listing`: Remove parking space listings
- `set_notification_settings`: Configure user notification preferences
- `add_feed_to_listing`: Integrate IoT sensor feeds
- `confirm_parking`: Confirm parking space reservations
- `reserve`: Reserve parking spaces

### 2. React Native Web App (`panorama-parking-react-native-web/`)
- Cross-platform mobile and web application
- Built with React Native Web for universal compatibility
- Next.js integration for optimal web performance

### 3. LoRaWAN Sensor Integration
<div align="center">
  <img src="lorawan.png" alt="LoRaWAN Sensor Network" width="600">
</div>

- **Switchboard Feeds**: Real-time sensor data integration
- **Car Arrival Feed**: `J748azokS8cKaiGKgN5hsTsTuB1FJ1ikVNXKjq9DQnjg`
- **Car Departure Feed**: `9jfL52Gmudwee1RK8yuNguoZET7DMDqKSR6DePBJNXot`

## 📊 Architecture Diagram

<div align="center">
  <img src="protocol-overview.png" alt="Protocol Overview Architecture" width="800">
</div>



## 👥 User Stories

*Coming Soon: Detailed user stories for different user types including parking space owners, renters, and administrators*

## 🎥 Demo Videos

*Coming Soon: Step-by-step demo videos for each user flow including:*
- *Parking space listing creation*
- *Space discovery and reservation*
- *Real-time sensor updates*
- *Payment processing*
- *Mobile app usage*

## 🚀 Upcoming Features

### MagicBlock Integration
*Coming Soon: Integration with [MagicBlock Engine](https://docs.magicblock.gg/pages/get-started/use-cases/depin) for enhanced DePIN capabilities*

**Benefits:**
- **Real-Time, Low-Cost Transactions** – Eliminates bottlenecks, enabling seamless on-chain infrastructure
- **Scalable & Efficient** – Optimized for high throughput and low fees, making microtransactions viable
- **Verifiable & Trustless** – Provides a tamper-proof, open ledger for transparent and decentralized infrastructure

**Addresses Current Limitations:**
- Solves scalability challenges for high transaction loads
- Reduces gas fees for IoT integrations and microtransactions
- Eliminates latency and finality delays for critical real-time applications

## 🚀 Getting Started

### Prerequisites
- Node.js (v16 or higher)
- Rust and Cargo
- Solana CLI tools
- Anchor Framework

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/DePIN-Panorama-Parking.git
   cd DePIN-Panorama-Parking
   ```

2. **Install dependencies**
   ```bash
   # Install Anchor program dependencies
   cd anchor-panorama-parking
   cargo build
   
   # Install React Native app dependencies
   cd ../panorama-parking-react-native-web
   yarn install
   ```

3. **Configure Solana**
   ```bash
   solana config set --url devnet
   solana airdrop 2
   ```

4. **Deploy the smart contract**
   ```bash
   cd ../anchor-panorama-parking
   anchor build
   anchor deploy
   ```

5. **Run the application**
   ```bash
   cd ../panorama-parking-react-native-web
   yarn dev
   ```

## 🔧 Development

### Smart Contract Development
```bash
cd anchor-panorama-parking
anchor test
anchor build
anchor deploy
```

### Frontend Development
```bash
cd panorama-parking-react-native-web
yarn dev             # Development server
yarn build           # Production build
yarn start           # Production server
```

## 📱 Mobile App

<div align="center">
  <img src="QR-android-apk.png" alt="Android APK QR Code" width="150" height="150">
  <p><em>Scan to download the Android APK</em></p>
</div>

## 🌐 Live Demo

- **Smart Contract Explorer**: [View on Solana Explorer](https://explorer.solana.com/address/FXUQwDsKJNrYFsfiUokPbH4BSrZtoC9m8HpoiMvYxtSE?cluster=devnet)
- **Program IDL**: [Anchor Program Interface](https://explorer.solana.com/address/FXUQwDsKJNrYFsfiUokPbH4BSrZtoC9m8HpoiMvYxtSE/anchor-program?cluster=devnet)

## 🧪 Testing

Run the comprehensive test suite:
```bash
cd anchor-panorama-parking
anchor test
```

The test suite includes examples of:
- Creating parking listings
- Updating listing information
- Managing notification settings
- Sensor feed integration
- Parking confirmations and reservations



## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🏆 Hackathon Submissions

This project was developed for multiple hackathons and demonstrates innovative use of:
- Solana blockchain technology
- DePIN (Decentralized Physical Infrastructure Networks)
- LoRaWAN sensor integration
- Cross-platform mobile development

### Breakout Hackathon
- **Event**: Breakout Hackathon
- **Focus**: Smart contract development and DePIN infrastructure

### Solana Mobile Hackathon
- **Event**: First Ever Solana Mobile Hackathon
- **Focus**: Mobile-first blockchain applications and Solana Mobile Stack integration


---

<div align="center">
  <p><strong>Built with ❤️ on Solana</strong></p>
  <p><em>Revolutionizing parking, one space at a time</em></p>
</div>
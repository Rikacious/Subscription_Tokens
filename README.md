# Subscription Tokens

## Project Description

Subscription Tokens is a basic Soroban smart contract built on the Stellar network that manages time-based subscriptions on-chain. It allows users to subscribe to a service for a fixed period, while the contract admin controls important settings like subscription price and duration.

This project is a good starting point for decentralized membership systems, premium communities, SaaS access control, token-gated apps, and recurring on-chain service models.

---

## What it does

The contract allows users to register and maintain an active subscription directly on Stellar using Soroban smart contracts.

It supports:
- Initializing the contract with an admin
- Setting a subscription price
- Setting a subscription duration
- Subscribing or renewing a subscription
- Checking whether a subscription is active
- Reading a user’s subscription details

Each subscriber has an on-chain record containing:
- `active` status
- `expires_at` timestamp

---

## Features

- Built with **Rust** and **Soroban SDK**
- On-chain subscription tracking
- Renewable subscriptions
- Admin-controlled price management
- Admin-controlled duration management
- Active/inactive subscription checks
- Event emission for subscription activity
- Simple architecture for easy extension

---

## Deployed Smart Contract Link

**Subscription Tokens**

> Add your deployed contract link here after deployment.

Example format:

[View Subscription Tokens Contract](https://stellar.expert/explorer/testnet/contract/YOUR_CONTRACT_ID)

---

## Core Functions

### `initialize(admin, price, duration)`
Initializes the contract and stores:
- Admin address
- Subscription price
- Subscription duration in seconds

### `subscribe(user)`
Lets a user subscribe or renew their subscription.

### `is_active(user)`
Returns `true` if the subscription is still valid.

### `get_subscription(user)`
Returns the subscription info for a user.

### `get_price()`
Returns the current subscription price.

### `set_price(admin, new_price)`
Allows the admin to update the subscription price.

### `get_duration()`
Returns the current subscription duration.

### `set_duration(admin, new_duration)`
Allows the admin to update the subscription duration.

---

## Use Cases

- Premium content platforms
- DAO memberships
- Paid communities
- Learning platforms
- Research subscriptions
- Creator access passes

---

## Future Improvements

- Accept real Stellar token payments
- Add multiple subscription tiers
- Add cancellation support
- Add monthly and yearly plans
- Add NFT-based proof of subscription
- Add withdrawal logic for admin revenue

---

## Tech Stack

- **Stellar**
- **Soroban**
- **Rust**

---

## License

MIT

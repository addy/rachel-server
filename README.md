# Rachel's Portfolio API

# Dev Setup

## Prerequisites
- Rust - use [rustup](https://rustup.rs/)
- Install dependencies
  - Run `cargo install`

## Configure environment
- Copy the `.env.example` file to `.env`
- Replace the values in the `.env` file with your own

1. DATABASE_URL
    - This should be the path to your SQLite DB
    - Defaults to `art.db`
2. STRIPE_ACCESS_TOKEN
    - This should be your private Stripe key, which looks like `pk_...`
3. SMTP_HOST
    - The host of your SMTP server, like `smtp.google.com`
4. SMTP_USER
    - Username to your SMTP server
5. SMTP_PASS
    - Password to your SMTP server
6. TO_EMAIL
    - Ideally your email, this is where all contact requests are sent to.

## Building
- Dev Build
  - Run `cargo build`
- Release Build
  - Run `cargo build --release`
- Running
  - Run `cargo run`

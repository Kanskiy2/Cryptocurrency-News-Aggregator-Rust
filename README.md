# Crypto News Aggregator

## Title
A Rust-based application for aggregating cryptocurrency news using NewsAPI.

## Usage

### Prerequisites:
1. Make sure you have [Rust](https://www.rust-lang.org/) installed.
2. You will need an API key from [NewsAPI](https://newsapi.org/).
3. Create a `.env` file in the root directory and add your `NEWSAPI_KEY` like so:
    ```
    NEWSAPI_KEY=your_api_key_here
    ```

### Steps to run:
1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/crypto_news_aggregator.git
    cd crypto_news_aggregator
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Run the server:
    ```bash
    cargo run
    ```

4. Open your browser and navigate to `http://127.0.0.1:8080/news/{coin}`, replacing `{coin}` with the cryptocurrency name (e.g., `bitcoin`, `ethereum`).

## Demo

![Demo](demo-screenshot.png)

## Examples

- Fetching news for Bitcoin:
  ```bash
  curl http://127.0.0.1:8080/news/bitcoin

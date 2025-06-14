# Dead Drop Frontend

A modern, privacy-preserving platform for reporting vulnerabilities in deployed smart contracts. Powered by advanced cryptography—Oblivious Message Retrieval (OMR) and PerfOMR—for end-to-end encrypted, metadata-private bug reporting.

## Features
- **End-to-end encrypted bug reporting**
- **Metadata privacy**: Only the intended developer can decrypt their reports
- **Modern, responsive UI** with dark/light mode
- **Technical details and performance metrics**
- **Open source and easy to integrate with custom OMR backends**

## Tech Stack
- [Next.js](https://nextjs.org/) (React, TypeScript)
- [Tailwind CSS](https://tailwindcss.com/)
- [PerfOMR/OMR cryptography](https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval/tree/perfomr) (backend integration)

## Getting Started

1. **Clone the repo:**
   ```sh
   git clone https://github.com/cynthwangg/deaddropic3.git
   cd deaddropic3
   ```
2. **Install dependencies:**
   ```sh
   npm install
   ```
3. **Run the development server:**
   ```sh
   npm run dev
   ```
4. **Open your browser:**
   - [http://localhost:3000](http://localhost:3000)

## Project Structure
- `/` — Landing and bug report form
- `/dashboard` — Developer dashboard for retrieving bug reports
- `/about` — Project overview and trust signals
- `/technical` — Technical details, algorithms, and performance

## Technical Details
- See [`/technical`](./src/app/technical/page.tsx) in the app
- [PerfOMR ePrint Paper (IACR 2024/204)](https://eprint.iacr.org/2024/204)
- [PerfOMR GitHub Repository](https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval/tree/perfomr)

## Integration
- See [`INTEGRATION_GUIDE.md`](./INTEGRATION_GUIDE.md) for how to connect this frontend to your OMR/TEE backend.

## Credits
- Inspired by [PerfOMR: Oblivious Message Retrieval with Reduced Communication and Computation](https://eprint.iacr.org/2024/204) by Zeyu Liu, Eran Tromer, Yunhao Wang
- Frontend by Cynthia Wang and contributors

## License
MIT (or specify your license)

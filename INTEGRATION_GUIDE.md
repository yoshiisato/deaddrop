# Integration Guide: Connecting the Frontend to Your Backend APIs

This guide explains how to connect the provided Next.js frontend to your custom backend for secure bug reporting using Oblivious Message Retrieval (OMR).

---

## 1. Overview

The frontend is designed to be backend-agnostic. You can connect it to any API that implements the required endpoints for:
- Submitting bug reports (as ciphertext)
- Retrieving bug reports for developers (using OMR)

---

## 2. Bug Reporter Flow (User Submits Bug)

**Frontend:**
- Located in `src/app/page.tsx` (the main page).
- When the user submits the form, an API call should be made to your backend.

**What to do:**
- Replace the `console.log('Form submitted:', formData);` in the `handleSubmit` function with an API call, e.g.:
  ```ts
  await fetch('/api/submit-bug', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(formData),
  });
  ```
- The backend should:
  1. Encrypt the bug report using the developer's public key (or OMR logic).
  2. Store the resulting ciphertext in a database (the "inbox").

**Backend Endpoint Example:**
- `POST /api/submit-bug`
  - Request body: `{ contractAddress, severity, description, proof }`
  - Response: `{ success: true }`

---

## 3. Developer Dashboard Flow (Developer Retrieves Bugs)

**Frontend:**
- Located in `src/app/dashboard/page.tsx`.
- When the developer connects their wallet and clicks to retrieve bugs, an API call should be made to your backend.

**What to do:**
- Replace the mock data in `handleConnectWallet` with an API call, e.g.:
  ```ts
  const res = await fetch(`/api/get-bugs?wallet=${walletAddress}`);
  const bugs = await res.json();
  setBugReports(bugs);
  ```
- The backend should:
  1. Run the OMR protocol to scan the database for messages the developer can decrypt.
  2. Return the decrypted bug reports to the frontend.

**Backend Endpoint Example:**
- `GET /api/get-bugs?wallet=0x...`
  - Response: `[{ id, contractAddress, severity, description, proof, timestamp }, ...]`

---

## 4. Where to Add API Calls

- **Bug Submission:**
  - Edit `src/app/page.tsx`, inside the `handleSubmit` function.
- **Bug Retrieval:**
  - Edit `src/app/dashboard/page.tsx`, inside the `handleConnectWallet` function.

---

## 5. Backend Implementation Notes

- You can use Next.js API routes (e.g., `src/pages/api/submit-bug.ts`) or an external server.
- All cryptography (encryption, OMR, decryption) should be handled in the backend.
- The frontend only sends and receives JSON data.
- Make sure to handle authentication and authorization as needed (e.g., wallet signature verification).

---

## 6. Example API Response Formats

- **Submit Bug:**
  ```json
  { "success": true }
  ```
- **Get Bugs:**
  ```json
  [
    {
      "id": 1,
      "contractAddress": "0x1234...5678",
      "severity": "high",
      "description": "Sample bug report",
      "proof": "Sample proof",
      "timestamp": "2024-06-01T12:00:00Z"
    }
  ]
  ```

---

## 7. Next Steps

- Implement the backend endpoints and OMR logic.
- Update the frontend to call your real APIs.
- (Optional) Add authentication, error handling, and UI improvements as needed.

---

**Questions?**
Feel free to ask for code samples or further integration help! 
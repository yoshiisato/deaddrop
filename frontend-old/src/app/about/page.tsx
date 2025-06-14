import Link from 'next/link';

export default function AboutPage() {
  return (
    <main className="bg-white dark:bg-gray-900 min-h-screen py-16 px-4">
      <div className="max-w-2xl mx-auto bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 rounded shadow p-8">
        <h1 className="text-3xl font-bold mb-6 text-indigo-800 dark:text-indigo-300">About Dead Drop</h1>
        <p className="mb-4 text-lg">
          <strong>Dead Drop</strong> is a secure, privacy-preserving platform for reporting vulnerabilities in deployed smart contracts. It leverages advanced cryptography—Oblivious Message Retrieval (OMR)—to ensure that:
        </p>
        <ul className="list-disc pl-6 mb-4">
          <li>Bug reports are encrypted and stored anonymously.</li>
          <li>Only the intended developer can retrieve and decrypt their reports.</li>
          <li>No one (not even the platform) can see which developer a report is for.</li>
        </ul>
        <p className="mb-4">
          This project is open source and designed for the security community. We believe in responsible disclosure and protecting both researchers and developers.
        </p>
        <div className="mb-6 flex flex-wrap gap-4">
          <span className="inline-block bg-green-100 text-green-800 px-3 py-1 rounded-full text-xs font-semibold">End-to-end encrypted</span>
          <span className="inline-block bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-xs font-semibold">Powered by OMR</span>
          <span className="inline-block bg-gray-100 text-gray-800 px-3 py-1 rounded-full text-xs font-semibold">Open Source</span>
        </div>
        <div className="flex gap-6">
          <Link href="https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval" target="_blank" rel="noopener noreferrer" className="text-indigo-700 dark:text-indigo-300 hover:underline font-medium">GitHub Repository</Link>
          <Link href="/INTEGRATION_GUIDE.md" target="_blank" rel="noopener noreferrer" className="text-indigo-700 dark:text-indigo-300 hover:underline font-medium">Integration Guide</Link>
        </div>
      </div>
    </main>
  );
} 
'use client';

import { useState } from 'react';
import Link from 'next/link';

export default function BugReporter() {
  const [formData, setFormData] = useState({
    description: '',
    proof: '',
    contractAddress: '',
    severity: 'medium',
  });
  const [success, setSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSuccess(false);
    try {
      // TODO: Implement submission logic with OMR
      console.log('Form submitted:', formData);
      setSuccess(true);
      setFormData({
        description: '',
        proof: '',
        contractAddress: '',
        severity: 'medium',
      });
    } catch (err) {
      setError('Failed to submit bug report. Please try again.');
    }
  };

  return (
    <main className="min-h-screen p-8 bg-gray-50">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-3xl font-bold mb-4 text-gray-900">Submit Bug Report</h1>
        <div className="mb-6 p-4 bg-indigo-50 border-l-4 border-indigo-400 rounded">
          <p className="text-indigo-900 mb-2 font-semibold">Technical Summary</p>
          <p className="text-indigo-800 text-sm">
            Your bug report will be <span className="font-bold">end-to-end encrypted</span> and submitted using <span className="font-mono">Oblivious Message Retrieval (OMR)</span>. Only the intended developer can decrypt it. <Link href="/technical" className="text-indigo-700 underline ml-1">Learn more about OMR</Link>.
          </p>
        </div>
        {success && (
          <div className="mb-4 p-3 rounded bg-green-100 text-green-800 border border-green-200">
            Bug report submitted successfully!
          </div>
        )}
        {error && (
          <div className="mb-4 p-3 rounded bg-red-100 text-red-800 border border-red-200">
            {error}
          </div>
        )}
        
        <form onSubmit={handleSubmit} className="space-y-6 bg-white p-6 rounded-lg shadow">
          <div>
            <label htmlFor="contractAddress" className="block text-sm font-medium text-gray-700">
              Smart Contract Address
              <span className="ml-1 text-xs text-gray-400" title="Ethereum address of the contract">ⓘ</span>
            </label>
            <input
              type="text"
              id="contractAddress"
              value={formData.contractAddress}
              onChange={(e) => setFormData({ ...formData, contractAddress: e.target.value })}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono"
              placeholder="0x..."
              required
            />
          </div>

          <div>
            <label htmlFor="severity" className="block text-sm font-medium text-gray-700">
              Severity Level
              <span className="ml-1 text-xs text-gray-400" title="How critical is the bug?">ⓘ</span>
            </label>
            <select
              id="severity"
              value={formData.severity}
              onChange={(e) => setFormData({ ...formData, severity: e.target.value })}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
            >
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="critical">Critical</option>
            </select>
          </div>

          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700">
              Bug Description
              <span className="ml-1 text-xs text-gray-400" title="You can use markdown and code blocks.">ⓘ</span>
            </label>
            <textarea
              id="description"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              rows={4}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono"
              placeholder="Describe the vulnerability in detail. Markdown and code supported."
              required
            />
          </div>

          <div>
            <label htmlFor="proof" className="block text-sm font-medium text-gray-700">
              Proof of Concept
              <span className="ml-1 text-xs text-gray-400" title="You can use markdown and code blocks.">ⓘ</span>
            </label>
            <textarea
              id="proof"
              value={formData.proof}
              onChange={(e) => setFormData({ ...formData, proof: e.target.value })}
              rows={6}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono"
              placeholder="Provide a detailed proof of concept. Markdown and code supported."
              required
            />
          </div>

          <button
            type="submit"
            className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            Submit Report
          </button>
        </form>
      </div>
    </main>
  );
}

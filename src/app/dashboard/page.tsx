'use client';

import { useState } from 'react';

export default function DeveloperDashboard() {
  const [walletAddress, setWalletAddress] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [bugReports, setBugReports] = useState<any[]>([]);
  const [scanStatus, setScanStatus] = useState<'idle' | 'scanning' | 'done'>('idle');

  const handleConnectWallet = async () => {
    setIsLoading(true);
    setScanStatus('scanning');
    try {
      const res = await fetch('/api/get-bugs?wallet=' + encodeURIComponent(walletAddress));
      const data = await res.json();
      setBugReports(data);
      setScanStatus('done');
    } catch (error) {
      setScanStatus('idle');
      setBugReports([]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main className="min-h-screen p-8 bg-gray-50">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-4 text-gray-900">Developer Dashboard</h1>
        <div className="mb-6 p-4 bg-indigo-50 border-l-4 border-indigo-400 rounded">
          <p className="text-indigo-900 mb-2 font-semibold">OMR Scan Summary</p>
          <p className="text-indigo-800 text-sm">
            When you connect your wallet, the system runs an <span className="font-mono">Oblivious Message Retrieval (OMR)</span> scan over the encrypted bug report database. Only you can decrypt your messages. <a href="/technical" className="text-indigo-700 underline ml-1">Learn more</a>.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow mb-8">
          <h2 className="text-xl font-semibold mb-4">Connect Wallet</h2>
          <div className="flex gap-4">
            <input
              type="text"
              value={walletAddress}
              onChange={(e) => setWalletAddress(e.target.value)}
              placeholder="Enter your wallet address"
              className="flex-1 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono"
            />
            <button
              onClick={handleConnectWallet}
              disabled={isLoading}
              className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
            >
              {isLoading ? 'Scanning...' : 'Connect & Scan'}
            </button>
          </div>
          {scanStatus === 'scanning' && (
            <div className="mt-4 text-indigo-700 flex items-center gap-2">
              <svg className="animate-spin h-5 w-5 text-indigo-700" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle><path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path></svg>
              Running OMR scan...
            </div>
          )}
          {scanStatus === 'done' && (
            <div className="mt-4 text-green-700">OMR scan complete. Decrypted bug reports are shown below.</div>
          )}
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Bug Reports</h2>
          {bugReports.length === 0 ? (
            <p className="text-gray-500">No bug reports found. Connect your wallet to view reports.</p>
          ) : (
            <div className="space-y-4">
              {bugReports.map((report) => (
                <div key={report.id} className="border rounded-lg p-4 bg-gray-50">
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="font-medium font-mono">Contract: {report.contractAddress}</h3>
                    <span className={`px-2 py-1 rounded text-sm font-mono ${
                      report.severity === 'critical' ? 'bg-red-100 text-red-800' :
                      report.severity === 'high' ? 'bg-orange-100 text-orange-800' :
                      report.severity === 'medium' ? 'bg-yellow-100 text-yellow-800' :
                      'bg-green-100 text-green-800'
                    }`}>
                      {report.severity.charAt(0).toUpperCase() + report.severity.slice(1)}
                    </span>
                  </div>
                  <div className="mb-2 text-xs text-gray-500 font-mono">Digest: {report.digest}</div>
                  <p className="text-gray-700 mb-2 whitespace-pre-line font-mono">{report.description}</p>
                  <div className="bg-gray-100 p-3 rounded">
                    <h4 className="font-medium mb-1">Proof of Concept:</h4>
                    <p className="text-xs text-gray-700 whitespace-pre-wrap font-mono">{report.proof}</p>
                  </div>
                  <p className="text-xs text-gray-500 mt-2 font-mono">
                    Received: {new Date(report.timestamp).toLocaleString()}
                  </p>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </main>
  );
} 
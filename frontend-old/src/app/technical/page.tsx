import Link from 'next/link';

export default function TechnicalPage() {
  return (
    <main className="bg-white dark:bg-gray-900 min-h-screen py-16 px-4">
      <div className="max-w-3xl mx-auto bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 rounded shadow p-8">
        <h1 className="text-3xl font-bold mb-6 text-indigo-800 dark:text-indigo-300">Technical Details: PerfOMR & OMR</h1>
        <section className="mb-8">
          <p className="mb-4">
            <strong>Dead Drop</strong> leverages advanced cryptographic primitives, including <span className="font-mono bg-indigo-100 px-1 rounded">Oblivious Message Retrieval (OMR)</span> and <span className="font-mono bg-indigo-100 px-1 rounded">PerfOMR</span>, to enable private, efficient, and scalable bug reporting for smart contracts.
          </p>
          <div className="flex flex-wrap gap-2 mb-4">
            <span className="inline-block bg-green-100 text-green-800 px-3 py-1 rounded-full text-xs font-semibold">Post-quantum secure</span>
            <span className="inline-block bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-xs font-semibold">End-to-end encrypted</span>
            <span className="inline-block bg-gray-100 text-gray-800 px-3 py-1 rounded-full text-xs font-semibold">Metadata-private</span>
          </div>
          <p className="mb-2">
            <span className="font-semibold">Why OMR?</span> Even end-to-end encrypted systems leak metadata. OMR lets an untrusted helper scan all messages and return a digest, so only the intended recipient can recover their messages—without revealing which messages are for whom.
          </p>
        </section>
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-2 text-indigo-700 dark:text-indigo-300">Actors & API</h2>
          <ul className="list-disc pl-6 mb-4">
            <li><strong>Sender:</strong> Posts (payload, clue) pairs to a public board.</li>
            <li><strong>Recipient:</strong> Publishes a clue key and later sends a detection key to the detector.</li>
            <li><strong>Detector:</strong> Computes a digest from the board and the detection key.</li>
          </ul>
          <div className="overflow-x-auto mb-4">
            <table className="min-w-full text-sm text-left border border-gray-200 dark:border-gray-700">
              <thead className="bg-indigo-50 dark:bg-gray-800">
                <tr>
                  <th className="px-3 py-2 font-semibold">Algorithm</th>
                  <th className="px-3 py-2 font-semibold">Purpose</th>
                </tr>
              </thead>
              <tbody>
                <tr><td className="px-3 py-2 font-mono">GenParam</td><td className="px-3 py-2">Set global parameters and error budgets</td></tr>
                <tr><td className="px-3 py-2 font-mono">KeyGen</td><td className="px-3 py-2">Generate recipient keys</td></tr>
                <tr><td className="px-3 py-2 font-mono">GenClue</td><td className="px-3 py-2">Sender encrypts a clue for each payload</td></tr>
                <tr><td className="px-3 py-2 font-mono">Retrieve</td><td className="px-3 py-2">Detector produces a compact digest</td></tr>
                <tr><td className="px-3 py-2 font-mono">Decode</td><td className="px-3 py-2">Recipient extracts all her messages</td></tr>
              </tbody>
            </table>
          </div>
        </section>
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-2 text-indigo-700 dark:text-indigo-300">Performance Snapshot (N=2¹⁹, k̄=50)</h2>
          <div className="overflow-x-auto mb-4">
            <table className="min-w-full text-sm text-left border border-gray-200 dark:border-gray-700">
              <thead className="bg-indigo-50 dark:bg-gray-800">
                <tr>
                  <th className="px-3 py-2 font-semibold">Metric</th>
                  <th className="px-3 py-2 font-semibold">OMRp2</th>
                  <th className="px-3 py-2 font-semibold">PerfOMR1</th>
                  <th className="px-3 py-2 font-semibold">PerfOMR2</th>
                </tr>
              </thead>
              <tbody>
                <tr><td className="px-3 py-2">Detector time</td><td className="px-3 py-2">109 ms</td><td className="px-3 py-2">7.9 ms</td><td className="px-3 py-2">39.6 ms</td></tr>
                <tr><td className="px-3 py-2">Clue key</td><td className="px-3 py-2">132 kB</td><td className="px-3 py-2">2.1 kB</td><td className="px-3 py-2">0.56 kB</td></tr>
                <tr><td className="px-3 py-2">Clue size</td><td className="px-3 py-2">956 B</td><td className="px-3 py-2">2.18 kB</td><td className="px-3 py-2">583 B</td></tr>
                <tr><td className="px-3 py-2">Digest / msg</td><td className="px-3 py-2">1.08 B</td><td className="px-3 py-2">2.71 B (v=8)</td><td className="px-3 py-2">1.08 B</td></tr>
                <tr><td className="px-3 py-2">Recipient time</td><td className="px-3 py-2">20 ms</td><td className="px-3 py-2">37 ms</td><td className="px-3 py-2">20 ms</td></tr>
              </tbody>
            </table>
          </div>
          <p className="mb-2">PerfOMR achieves up to <span className="font-semibold">13.8× faster</span> detection and <span className="font-semibold">235× smaller</span> clue keys compared to previous OMR schemes, while maintaining post-quantum security and privacy guarantees.</p>
        </section>
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-2 text-indigo-700 dark:text-indigo-300">References</h2>
          <ul className="list-disc pl-6">
            <li><Link href="https://eprint.iacr.org/2024/204" className="text-indigo-700 dark:text-indigo-300 underline" target="_blank">PerfOMR ePrint Paper (IACR 2024/204)</Link></li>
            <li><Link href="https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval/tree/perfomr" className="text-indigo-700 dark:text-indigo-300 underline" target="_blank">PerfOMR GitHub Repository</Link></li>
            <li><Link href="https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval/tree/eu_dos_perfomr_anony" className="text-indigo-700 dark:text-indigo-300 underline" target="_blank">EU DoS PerfOMR Anony branch</Link></li>
          </ul>
        </section>
      </div>
    </main>
  );
} 
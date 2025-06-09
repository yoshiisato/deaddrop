import Link from 'next/link';
import { ShieldCheckIcon, LockClosedIcon, UserGroupIcon } from '@heroicons/react/24/outline';

export default function HomePage() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-indigo-900 to-indigo-600 flex flex-col items-center justify-center text-white">
      <div className="w-full max-w-3xl px-6 py-16 flex flex-col items-center">
        <h1 className="text-5xl font-extrabold mb-4 tracking-tight">Dead Drop</h1>
        <p className="text-xl mb-8 text-center max-w-xl">
          Stealthy, private, and efficient smart-contract vulnerability reporting.<br />
          <span className="text-indigo-200">Powered by Oblivious Message Retrieval (OMR).</span>
        </p>
        <div className="flex gap-4 mb-12">
          <Link href="/" className="bg-white text-indigo-700 px-6 py-3 rounded-lg font-semibold shadow hover:bg-indigo-100 transition">Report a Bug</Link>
          <Link href="/dashboard" className="bg-indigo-700 border border-white px-6 py-3 rounded-lg font-semibold shadow hover:bg-indigo-800 transition">Developer Dashboard</Link>
        </div>
        <section className="w-full bg-white/10 rounded-xl p-8 mt-8">
          <h2 className="text-2xl font-bold mb-6 text-white text-center">How It Works</h2>
          <div className="flex flex-col md:flex-row gap-8 justify-center items-center">
            <div className="flex flex-col items-center text-center">
              <ShieldCheckIcon className="h-12 w-12 mb-2 text-indigo-200" />
              <h3 className="font-semibold mb-1">1. Submit Privately</h3>
              <p className="text-indigo-100">Researchers submit encrypted bug reports—no one can see the recipient.</p>
            </div>
            <div className="flex flex-col items-center text-center">
              <LockClosedIcon className="h-12 w-12 mb-2 text-indigo-200" />
              <h3 className="font-semibold mb-1">2. Secure Storage</h3>
              <p className="text-indigo-100">Bugs are stored securely and anonymously in the OMR-powered database.</p>
            </div>
            <div className="flex flex-col items-center text-center">
              <UserGroupIcon className="h-12 w-12 mb-2 text-indigo-200" />
              <h3 className="font-semibold mb-1">3. Only You Retrieve</h3>
              <p className="text-indigo-100">Only the right developer can retrieve and decrypt their bug reports.</p>
            </div>
          </div>
        </section>
      </div>
    </main>
  );
} 
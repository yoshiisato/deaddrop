import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Navigation from "@/components/Navigation";
import Link from "next/link";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Dead Drop - Stealthy Smart Contract Vulnerability Reporting",
  description: "A secure platform for reporting smart contract vulnerabilities using Oblivious Message Retrieval (OMR)",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className + " bg-gray-50 dark:bg-gray-900 min-h-screen flex flex-col text-gray-900 dark:text-gray-100"}>
        <Navigation />
        <div className="flex-1 flex flex-col">{children}</div>
        <footer className="bg-white dark:bg-gray-950 border-t py-4 mt-8 text-center text-gray-500 dark:text-gray-400 text-sm">
          <div className="flex flex-col sm:flex-row justify-center items-center gap-2">
            <span>© {new Date().getFullYear()} Dead Drop</span>
            <span>·</span>
            <Link href="https://github.com/ObliviousMessageRetrieval/ObliviousMessageRetrieval" className="hover:underline" target="_blank" rel="noopener noreferrer">GitHub</Link>
            <span>·</span>
            <span>Powered by OMR</span>
          </div>
        </footer>
      </body>
    </html>
  );
}

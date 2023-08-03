import { ThemeToggle } from "@/components/theme-toggle";
import "./globals.css";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import Link from "next/link";
import { useCallback, useState } from "react";
import { PiLightbulbFill, PiLightbulbLight } from "react-icons/pi";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Basalytics Dashboard",
  description: "View your analytics data",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body
        className={`min-h-screen flex flex-col justify-start ${inter.className} bg-base-300`}
      >
        <header className="navbar bg-base-100 shadow-lg mb-8">
          <div className="navbar-start">
            <Link className="btn btn-ghost text-xl normal-case" href="/">
              Basalytics
            </Link>
          </div>
          <div className="navbar-end flex flex-row gap-2">
            <ThemeToggle />
            <Link className="btn btn-primary" href="/login">
              Log in
            </Link>
          </div>
        </header>
        <main className="grow mx-4">{children}</main>
        <footer className="p-4 shadow-above-lg bg-base-200 opacity-90 mt-8">
          Basalytics is open source software licensed under AGPLv3.
        </footer>
      </body>
    </html>
  );
}

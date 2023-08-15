"use client";
import { useAuth } from "@/hooks/auth";
import Link from "next/link";

export function LoginButton() {
  const { token, logout } = useAuth();

  if (token) {
    return (
      <>
        <Link className="btn btn-primary" href="/dashboard/">
          Dashboard
        </Link>
        <button className="btn btn-primary" onClick={logout}>
          Log out
        </button>
      </>
    );
  }

  return (
    <Link className="btn btn-primary" href="/login/">
      Log in
    </Link>
  );
}

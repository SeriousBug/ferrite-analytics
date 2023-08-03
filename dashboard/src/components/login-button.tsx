"use client";
import { useAuth } from "@/hooks/auth";
import Link from "next/link";

export function LoginButton() {
  const { token } = useAuth();

  if (token) {
    return (
      <Link className="btn btn-primary" href="/dashboard">
        Dashboard
      </Link>
    );
  }

  return (
    <Link className="btn btn-primary" href="/login">
      Log in
    </Link>
  );
}

"use client";
import { useAuth } from "@/hooks/auth";
import { useRouter } from "next/navigation";
import { useCallback, useEffect, useState } from "react";

export default function Login() {
  const router = useRouter();
  const { token, login } = useAuth();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  // Redirect already logged in users to the dashboard
  useEffect(() => {
    console.log("token", token);
    if (token) router.replace("/dashboard");
  }, [router, token]);

  const handleLogin = useCallback(async () => {
    try {
      await login({ username, password });
    } catch (err) {
      // TODO display the error to the user
      console.error(err);
    }
  }, [login, password, username]);

  return (
    <div className="flex flex-col p-4 max-w-md mx-auto gap-4 shadow-xl bg-base-100 rounded-box">
      <div className="form-control w-full">
        <label htmlFor="username" className="label">
          <span className="label-text">Username</span>
        </label>
        <input
          id="username"
          type="text"
          placeholder="mhamilton"
          className="input input-bordered"
          value={username}
          onChange={useCallback(
            (e: React.ChangeEvent<HTMLInputElement>) =>
              setUsername(e.target.value),
            [setUsername],
          )}
        />
      </div>
      <div className="form-control w-full">
        <label htmlFor="password" className="label">
          <span className="label-text">Password</span>
        </label>
        <input
          id="password"
          type="password"
          placeholder="correct horse battery staple"
          className="input input-bordered"
          value={password}
          onChange={useCallback(
            (e: React.ChangeEvent<HTMLInputElement>) =>
              setPassword(e.target.value),
            [setPassword],
          )}
        />
      </div>
      <button className="btn btn-primary w-full mt-8" onClick={handleLogin}>
        Log in
      </button>
    </div>
  );
}

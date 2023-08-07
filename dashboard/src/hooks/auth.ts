import { z } from "zod";
import { LocalStorage, useLocalStorage } from "./localStorage";
import { useCallback } from "react";

const authSchema = z.object({
  token: z.string(),
});

export function useAuth() {
  const [storedToken, updateToken] = useLocalStorage(
    LocalStorage.Auth,
    authSchema.parse,
  );

  const login = useCallback(
    async (body: { username: string; password: string }) => {
      const response = await fetch("http://localhost:3000/api/auth/login", {
        method: "POST",
        body: JSON.stringify(body),
        headers: {
          "Content-Type": "application/json",
        },
      });
      const token = await response.text();
      updateToken({ token });
    },
    [updateToken],
  );

  const logout = useCallback(() => {
    updateToken(null);
  }, [updateToken]);

  return {
    token: storedToken?.token ?? null,
    login,
    logout,
  };
}

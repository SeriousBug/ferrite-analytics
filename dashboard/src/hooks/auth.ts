import { z } from "zod";
import { LocalStorage, useLocalStorage } from "./localStorage";
import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { API_BASE_URL } from "@/helpers/api";

const authSchema = z.object({
  token: z.string(),
});

export function useAuth() {
  const router = useRouter();
  const [storedToken, updateToken] = useLocalStorage(
    LocalStorage.Auth,
    authSchema.parse,
  );

  const login = useCallback(
    async (body: { username: string; password: string }) => {
      const response = await fetch(`${API_BASE_URL}/auth/login`, {
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
    router.push("/login");
  }, [router, updateToken]);

  return {
    token: storedToken?.token ?? null,
    login,
    logout,
  };
}

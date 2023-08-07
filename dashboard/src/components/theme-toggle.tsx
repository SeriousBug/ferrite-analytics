"use client";
import { LocalStorage, useLocalStorage } from "@/hooks/localStorage";
import { useCallback } from "react";
import { PiLightbulbBold } from "react-icons/pi";
import { z } from "zod";

const themeSchema = z.enum(["dark", "light"]).nullable();

export function ThemeToggle() {
  const [theme, setTheme] = useLocalStorage(
    LocalStorage.Theme,
    themeSchema.parse,
  );
  const toggleTheme = useCallback(() => {
    console.log("toggle");
    if (theme === "dark") {
      document.body.dataset.theme = "autumn";
      setTheme("light");
    } else {
      document.body.dataset.theme = "forest";
      setTheme("dark");
    }
  }, [theme, setTheme]);

  return (
    <div className="btn btn-ghost text-xl" onClick={toggleTheme}>
      <PiLightbulbBold />
    </div>
  );
}

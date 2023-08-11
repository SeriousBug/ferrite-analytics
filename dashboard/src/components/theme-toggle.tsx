"use client";
import { LocalStorage, useLocalStorage } from "@/hooks/localStorage";
import { useThemeColor } from "@/hooks/themeColor";
import { useCallback } from "react";
import { PiLightbulbBold } from "react-icons/pi";
import { z } from "zod";

const themeSchema = z.enum(["dark", "light"]).nullable();

export function ThemeToggle() {
  const { mutate: mutateTheme } = useThemeColor();
  const [theme, setTheme] = useLocalStorage(
    LocalStorage.Theme,
    themeSchema.parse,
  );
  const toggleTheme = useCallback(() => {
    if (theme === "dark") {
      document.body.dataset.theme = "autumn";
      setTheme("light");
    } else {
      document.body.dataset.theme = "forest";
      setTheme("dark");
    }
    mutateTheme();
  }, [theme, mutateTheme, setTheme]);

  return (
    <div
      aria-label="toggle theme"
      className="btn btn-ghost text-xl"
      onClick={toggleTheme}
    >
      <PiLightbulbBold />
    </div>
  );
}

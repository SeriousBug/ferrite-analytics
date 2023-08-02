"use client";
import { useCallback, useState } from "react";
import { PiLightbulbFill } from "react-icons/pi";

export function ThemeToggle() {
  const [isDark, setIsDark] = useState(false);
  const toggleTheme = useCallback(() => {
    setIsDark(!isDark);
    if (isDark) {
      document.body.dataset.theme = "autumn";
    } else {
      document.body.dataset.theme = "forest";
    }
  }, [isDark]);

  return (
    <div className="btn btn-ghost text-xl" onClick={toggleTheme}>
      <PiLightbulbFill />
    </div>
  );
}

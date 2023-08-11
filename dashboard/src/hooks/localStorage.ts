"use client";
import { useCallback, useEffect, useState } from "react";

export enum LocalStorage {
  Auth = "Auth",
  Theme = "Theme",
  Cards = "Cards",
}

export function useLocalStorage<T>(
  key: LocalStorage,
  parse: (value: unknown) => T,
): [T | null, (value: T | null) => void] {
  const [value, setValue] = useState<T | null>(null);

  useEffect(() => {
    const stored = localStorage.getItem(key);
    if (stored) {
      try {
        setValue(parse(JSON.parse(stored)));
      } catch (error) {
        // TODO: toast the error
        console.error("Failed to parsed data saved in localStorage", error);
        localStorage.removeItem(key);
      }
    }
  }, [key, parse]);

  const update = useCallback(
    (newValue: T | null) => {
      setValue(newValue);
      if (newValue === null) {
        localStorage.removeItem(key);
      } else {
        localStorage.setItem(key, JSON.stringify(parse(newValue)));
      }
    },
    [key, parse],
  );

  return [value, update];
}

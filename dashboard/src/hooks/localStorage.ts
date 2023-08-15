"use client";
import { useCallback, useEffect } from "react";
import useSWR from "swr";

export enum LocalStorage {
  Auth = "Auth",
  Theme = "Theme",
  Cards = "Cards",
}

export function useLocalStorage<T>(
  key: LocalStorage,
  parse: (value: unknown) => T,
): [T | null, (value: T | null) => void] {
  const fetchLocalStorageValue = useCallback(() => {
    const stored = localStorage.getItem(key);
    if (stored) {
      try {
        return parse(JSON.parse(stored));
      } catch (error) {
        // TODO: toast the error
        console.error("Failed to parsed data saved in localStorage", error);
        localStorage.removeItem(key);
      }
    }
  }, [key, parse]);

  const { data, mutate } = useSWR(
    "data:localStorage:" + key,
    fetchLocalStorageValue,
  );

  const update = useCallback(
    (newValue: T | null) => {
      if (newValue === null) {
        localStorage.removeItem(key);
      } else {
        localStorage.setItem(key, JSON.stringify(parse(newValue)));
      }
      mutate();
    },
    [key, mutate, parse],
  );

  // Reload if local storage changes on another tab
  useEffect(() => {
    const handleStorage = (event: StorageEvent) => {
      if (event.key === key) {
        mutate();
      }
    };
    window.addEventListener("storage", handleStorage);
    return () => window.removeEventListener("storage", handleStorage);
  }, [key, mutate]);

  return [data ?? null, update];
}

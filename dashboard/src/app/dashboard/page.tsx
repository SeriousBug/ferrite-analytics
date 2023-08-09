"use client";
import { useAuth } from "@/hooks/auth";
import { endOfDay, formatRFC3339, startOfDay, subDays } from "date-fns";
import { useCallback, useMemo } from "react";
import useSWRInfinite from "swr/infinite";
import { z } from "zod";
import { Line } from "react-chartjs-2";

import {
  Chart as ChartJS,
  LinearScale,
  CategoryScale,
  PointElement,
  LineElement,
} from "chart.js";
ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement);

type AnalyticsFilter =
  | { name: string; eq: string }
  | {
      and: AnalyticsFilter[];
    }
  | {
      or: AnalyticsFilter[];
    };

type AnalyticsRequest = {
  from_date: string;
  to_date: string;
  filter: AnalyticsFilter;
};

function useAnalyticsDataFetcher(name: string, eq: string) {
  return useCallback(
    async ([token, url, date]: [string, string, Date]) => {
      if (!token) throw new Error("No token");

      const body: AnalyticsRequest = {
        from_date: formatRFC3339(startOfDay(date)),
        to_date: formatRFC3339(endOfDay(date)),
        filter: {
          eq,
          name,
        },
      };

      const resp = await fetch(url, {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(body),
      });
      return z.coerce.number().parse(await resp.text());
    },
    [eq, name],
  );
}

function useAnalyticsData(name: string, eq: string) {
  const fetcher = useAnalyticsDataFetcher(name, eq);
  const { token } = useAuth();
  const today = useMemo(() => new Date(), []);

  const getKey = useCallback(
    (pageIndex: number) => {
      return [
        token,
        "http://localhost:3000/api/query/filter",
        subDays(today, pageIndex),
      ];
    },
    [today, token],
  );

  const { data, error } = useSWRInfinite(getKey, fetcher, {
    initialSize: 14,
    revalidateOnFocus: false,
    // Cache results for 10 minutes
    dedupingInterval: 1000 * 60 * 10,
  });
  return {
    data: useMemo(() => data?.reverse(), [data]),
    error,
  };
}

export default function Dashboard() {
  const { data, error } = useAnalyticsData("name", "tracking-pixel");

  console.log(error);

  return (
    <>
      <p>Hey</p>
      <Line
        data={{
          labels: [
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
          ],
          datasets: [{ data }],
        }}
      />
    </>
  );
}

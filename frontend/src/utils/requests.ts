import { getToken } from "./auth";
import { push } from "svelte-spa-router";

export const sendRequest = async <R>(
  url: string,
  method: "GET" | "POST" | "PUT",
  payload?: object
): Promise<R | null> => {
  try {
    const token = getToken();
    const res = await fetch(url, {
      headers: {
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...(payload ? { "Content-Type": "application/json" } : {}),
      },
      body: JSON.stringify(payload),
      method,
    });
    if (!res.ok) {
      return null;
    }
    return (await res.json()) as R;
  } catch (err) {
    await push("/errors/unexpected");
  }
  return null;
};

import jwtDecode from "jwt-decode";

const ACCESS_TOKEN = "ACCESS_TOKEN";

export const getToken = () => window.localStorage.getItem(ACCESS_TOKEN);

export const saveToken = (token: string) =>
  window.localStorage.setItem(ACCESS_TOKEN, token);

export type Claims = { sub: string; name: string; game: string };

export const getClaims = (): Claims => {
  const token = getToken();
  if (!token) {
    throw new Error("Token missing");
  }

  return jwtDecode<Claims>(token);
};

import { useQuery } from "react-query";

const fetchQuote = async (
  token_in: string,
  token_out: string,
  amount: string
) => {
  const quote = await (
    await fetch(
      `http://localhost:8080/quote/${token_in}/${token_out}/${amount}`
    )
  ).json();
  return quote;
};

const usePools = (token_in: string, token_out: string, amount: string) => {
  return useQuery("quote", () => fetchQuote(token_in, token_out, amount));
};

export { usePools, fetchQuote };

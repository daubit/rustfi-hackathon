import { useQuery } from "react-query";

const fetchQuote = async (
  token_in: string,
  token_out: string,
  amount: string
) => {
  console.log("Fetching quote!");
  const quote = await (
    await fetch(
      `http://localhost:8080/quote/${token_in}/${token_out}/${amount}`
    )
  ).json();
  return quote;
};

const useQuote = (token_in: string, token_out: string, amount: string) => {
  return useQuery("quote", () => fetchQuote(token_in, token_out, amount));
};

export { useQuote, fetchQuote };

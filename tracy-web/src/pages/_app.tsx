import "../styles/globals.css";
import type { AppProps } from "next/app";
import { ChakraProvider, ToastProvider } from "@chakra-ui/react";
import { QueryClient, QueryClientProvider } from "react-query";
import React from "react";

export const queryClient = new QueryClient();

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <QueryClientProvider client={queryClient}>
      <ChakraProvider>
        <Component {...pageProps} />
      </ChakraProvider>
    </QueryClientProvider>
  );
}

export default MyApp;

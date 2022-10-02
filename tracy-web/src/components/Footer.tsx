import Image from "next/image";
import React from "react";
import styles from "../styles/Home.module.css";

export const Footer = () => {
  return (
    <footer className={styles.footer}>
      <a href="https://daubit.org" target="_blank" rel="noopener noreferrer">
        Powered by{" "}
        <span className={styles.logo}>
          <Image
            src="/images/daubit-logo.png"
            alt="Daubt Logo"
            width={32}
            height={32}
          />
        </span>
      </a>
    </footer>
  );
};

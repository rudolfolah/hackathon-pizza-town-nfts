import React, {useEffect} from "react";
// import { ConnectSample } from 'components/ConnectSample';
// import { QuerySample } from 'components/QuerySample';
// import { TxSample } from 'components/TxSample';
import {useWallet, WalletStatus} from "@terra-money/wallet-provider";
import ConnectWalletButtons from "../components/ConnectWalletButtons";
import "./Main.css";

export default function About() {
  const { status } = useWallet();
  return (
    <section className="screen-main">
      <header>
        <h1>Pizza Town</h1>
      </header>
      <section>
        <header>
          <h2>How to mint</h2>
        </header>
        <section>
          <p>1. <a target="_blank" href="https://chrome.google.com/webstore/detail/terra-station/aiifbnbfobpmeekipheeijimdpnlpgpp/">Click here to download the Terra Station wallet for your browser</a></p>
          <p>2. Create your wallet using Terra Station.</p>
          <p>3. Connect your wallet.</p>
          <p>4. Press "Mint" to mint a new pizza!</p>
        </section>
      </section>
      <section>
        <ConnectWalletButtons />
      </section>
    </section>
  );
}

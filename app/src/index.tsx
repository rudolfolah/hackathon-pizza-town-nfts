import React from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";
import { NetworkInfo, WalletProvider } from '@terra-money/wallet-provider';

import About from 'screens/About';
import Main from 'screens/Main';
import './style.css';
import {TokenBalance} from "components/TokenBalance";
import ConnectWalletButtons from "./components/ConnectWalletButtons";
import GameButton from "./components/GameButton";
import Button from "./components/Button";

const mainnet = {
  name: 'mainnet',
  chainID: 'columbus-5',
  lcd: 'https://lcd.terra.dev',
};

const testnet = {
  name: 'bombay',
  chainID: 'bombay-12',
  lcd: 'https://bombay-lcd.terra.dev',
};

const walletConnectChainIds: Record<number, NetworkInfo> = {
  0: testnet,
  1: mainnet,
};

ReactDOM.render(
  <WalletProvider
    defaultNetwork={testnet}
    walletConnectChainIds={walletConnectChainIds}
  >
    <Router>
      <div id="nav">
        <Button to="/">Pizza Town</Button>
        <Button to="/about">About</Button>
        <ConnectWalletButtons />
      </div>
      <Switch>
        <Route path="/about">
          <About />
        </Route>
        <Route path="/">
          <Main />
        </Route>
      </Switch>
    </Router>
  </WalletProvider>,
  document.getElementById('root'),
);

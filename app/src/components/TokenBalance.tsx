import React, {useEffect, useMemo, useState} from "react";
import {LCDClient} from "@terra-money/terra.js";
import {useConnectedWallet} from "@terra-money/wallet-provider";

import {CONTRACT_ADDRESS} from "./constants";
import {BalanceResponse} from "../types";

export function TokenBalance() {
  const connectedWallet = useConnectedWallet();
  const lcd = useMemo(() => {
    if (!connectedWallet) {
      return null;
    }
    return new LCDClient({
      URL: connectedWallet.network.lcd,
      chainID: connectedWallet.network.chainID,
    })
  }, [connectedWallet]);
  const [ustBalance, setUstBalance] = useState<null | string>();
  useEffect(() => {
    if (connectedWallet && lcd) {
      lcd.bank.balance(connectedWallet.walletAddress).then(coins => {
        setUstBalance(coins.get("uusd")?.toString());
      });
    } else {
      setUstBalance("0");
    }
  }, [connectedWallet, lcd]);
  return (<div>
    <div>{ustBalance}</div>
  </div>);
}

import React, {useCallback, useEffect, useMemo, useState} from "react";
import {NftPizza, NftPie, PizzaTownInventoryResponse} from "../types";
import {CONTRACT_ADDRESS} from "../components/constants";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import './Main.css';
import LayeredImage from "../components/LayeredImage";
import GameButton from "../components/GameButton";

export default function Main() {
  const [pies, setPies] = useState<NftPie[]>();
  const [pizzas, setPizzas] = useState<NftPizza[]>();
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
  const queryInventory = () => {
    if (connectedWallet && lcd) {
      lcd.wasm.contractQuery<PizzaTownInventoryResponse>(CONTRACT_ADDRESS, {
        "inventory": {
          "address": connectedWallet.walletAddress,
        },
      }).then(result => {
        setPies(result.pies);
        setPizzas(result.pizzas);
      });
    } else {
      setPies([]);
      setPizzas([]);
    }
  };
  useEffect(() => {
    queryInventory();
  }, [connectedWallet, lcd]);

  const handleMintClick = () => {
    if (!connectedWallet || !lcd) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        mint_pizza: {},
      },
      {},
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uluna: 15000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log("Minted pizza");
      setTimeout(() => { queryInventory(); }, 4000);
      setTimeout(() => { queryInventory(); }, 8000);
    }).catch((error: unknown) => {
      console.error(error);
    });
  };
  const handleCombineClick = () => {
    if (!connectedWallet || !lcd) {
      return;
    }
    if (!pizzas || pizzas?.length < 2) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        mint_pie: {
          pizza_a_id: pizzas[0].id,
          pizza_b_id: pizzas[1].id,
        },
      },
      {},
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uluna: 15000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log("Minted pizza");
      setTimeout(() => { queryInventory(); }, 4000);
      setTimeout(() => { queryInventory(); }, 8000);
    }).catch((error: unknown) => {
      console.error(error);
    });
  };

  const size = 256;

  return (<div className="screen-main">
    <section className="collections">
      <section className="pizza-collection">
        <header>
          <section className="left">&nbsp;</section>
          <section className="center">
            <h3>{pies?.length} Pies</h3>
          </section>
          <section className="right">&nbsp;</section>
        </header>
        <section className="pizza-items-container">
          <section className="pizza-items-container-left">&nbsp;</section>
          <section className="pizza-items">
            {pies?.map(pizza =>
              <div key={pizza.id} className="pizza-item" style={{ width: size + 24, height: size + 24}}>
                <LayeredImage
                  layers={[`/assets/pies/${pizza.pie}.png`]}
                  height={size} width={size}
                />
              </div>
            )}
          </section>
          <section className="pizza-items-container-right">&nbsp;</section>
        </section>
        <footer>
          <section className="left">&nbsp;</section>
          <section className="center">&nbsp;</section>
          <section className="right">&nbsp;</section>
        </footer>
      </section>
      <section className="pizza-collection">
        <header>
          <section className="left">&nbsp;</section>
          <section className="center">
            <h3>{pizzas?.length} Pizzas</h3>
          </section>
          <section className="right">&nbsp;</section>
        </header>
        <section className="pizza-items-container">
          <section className="pizza-items-container-left">&nbsp;</section>
          <section className="pizza-items">
            {pizzas?.map(pizza =>
              <div key={pizza.id} className="pizza-item" style={{ width: size + 24, height: size + 24}}>
                <LayeredImage
                  layers={[
                    `/assets/backgrounds/${pizza.background}.jpg`,
                    `/assets/pizzas/${pizza.pizza}.png`,
                    `/assets/toppings/a${pizza.topping1}.png`,
                    `/assets/toppings/b${pizza.topping2}.png`,
                    `/assets/toppings/c${pizza.topping3}.png`,
                  ]}
                  height={size} width={size}
                />
              </div>
            )}
          </section>
          <section className="pizza-items-container-right">&nbsp;</section>
        </section>
        <footer>
          <section className="left">&nbsp;</section>
          <section className="center">&nbsp;</section>
          <section className="right">&nbsp;</section>
        </footer>
        </section>
    </section>
    <section className="sidebar">
      <section className="sidebar-mint">
        <GameButton onClick={handleMintClick}>
          Mint
        </GameButton>
      </section>
      <section className="sidebar-combine">
        <GameButton onClick={handleCombineClick}>
          Combine
        </GameButton>
      </section>
    </section>
  </div>);
}

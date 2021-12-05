import React, {useCallback, useEffect, useMemo, useState} from "react";
import {NftPizza, NftPie, PizzaTownInventoryResponse} from "../types";
import {CONTRACT_ADDRESS} from "../components/constants";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import './Main.css';
import {PIE_IMAGES, PIZZA_IMAGES} from "../constants";
import {Link} from "react-router-dom";
import Button from "../components/Button";
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
      setTimeout(() => { queryInventory(); }, 5000);
    }).catch((error: unknown) => {
      console.error(error);
    });
  };
  const executeSellDogOnMarket = (dog_id: string, price_in_uusd: string) => {
    if (!connectedWallet || !lcd) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        sell_dog_on_market: {
          dog_id: dog_id,
          price: price_in_uusd,
        },
      },
      { uusd: 1 },
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uusd: 200000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log("Dog listed for sale");
    }).catch((error: unknown) => {
      console.error(error);
    });
  }

  const size = 256;

  return (<div className="screen-main">
    <section className="collections">
      <section className="pie-collection">
        <header>Collect All Pies</header>
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
                    `/assets/backgrounds/0${pizza.background}.jpg`,
                    `/assets/pizzas/0${pizza.pizza}.png`,
                    `/assets/toppings/a0${pizza.topping1}.png`,
                    `/assets/toppings/b0${pizza.topping2}.png`,
                    `/assets/toppings/c0${pizza.topping3}.png`,
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
        <GameButton onClick={() => {}}>
          Select 2 Pizzas
        </GameButton>
        <GameButton onClick={() => {}}>
          Combine
        </GameButton>
      </section>
    </section>
  </div>);
}

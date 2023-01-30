import { useEffect, useState } from "react";
import "./App.css";
import { init, performRequestFromUi } from "./utils/namadaSdk/index";
function App() {
  // placeholder account to query the balance for
  const [accountToQuery, setAccountToQuery] = useState(
    "atest1v4ehgw36gsun2vpe8qcyg329xppnsdphxppy2333gscrgwpnggcrssecgfpy23fsx5eyxvjpqeulnk"
  );
  // balance that will be feched by the SDK
  const [balance, setBalance] = useState("");

  // we initialize the wasm module
  useEffect(() => {
    const asyncBlock = async () => {
      await init();
    };
    asyncBlock();
  }, []);

  // we trigger a function in Rust/WASM
  const fetchBalanceOfAddress = () => {
    const asyncBlock = async () => {
      const balance = await performRequestFromUi(accountToQuery);
      setBalance(balance);
    };
    asyncBlock();
  };

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: "flex", flexDirection: "column", width: "50%" }}>
          <input
            value={accountToQuery}
            onChange={(event) => {
              setAccountToQuery(event.target.value);
            }}
            style={{ height: "32px", margin: "0 0 16px" }}
          />
          <button
            onClick={() => {
              fetchBalanceOfAddress();
            }}
            style={{ height: "32px", margin: "0 0 16px" }}
          >
            query balance of address
          </button>
          <div>balance:</div>
          <div>NAM {balance} μ</div>
        </div>
      </header>
    </div>
  );
}

export default App;

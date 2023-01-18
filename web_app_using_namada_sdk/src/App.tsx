import { useEffect, useState } from "react";
import "./App.css";
import { init, performRequestFromUi } from "./utils/namadaSdk/index";
function App() {
  const [inputFieldContent, setInputFieldContent] = useState(
    "atest1d9khqw36gguyx3p4g4rrjvfcx9pnv32rx3q5vd3kx5cn2wzxgs6n2v3kxycnx32z8q6nqs2x5anmwx"
  );
  useEffect(() => {
    // we have to put this to async as we call async funcs
    const asyncBlock = async () => {
      await init();
    };
    asyncBlock();
  }, []);

  const fetchBalanceOfAddress = () => {
    // we have to put this to async as we call async funcs
    const asyncBlock = async () => {
      await performRequestFromUi(inputFieldContent);
    };
    asyncBlock();
  };

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: "flex", flexDirection: "column", width: "50%" }}>
          <input
            value={inputFieldContent}
            onChange={(event) => {
              setInputFieldContent(event.target.value);
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
        </div>
      </header>
    </div>
  );
}

export default App;

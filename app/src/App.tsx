import React from 'react';
import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';
import { Box, AppBar, Toolbar, Button, Typography } from '@mui/material';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import CreatorPage from './CreatorPage';
import InvestorPage from './InvestorPage';
import WalletContext from './WalletContext';

const App: React.FC = () => {
    return (
        <BrowserRouter>
            <WalletContext>
                {/* Навігаційна панель */}
                <AppBar position="static" sx={{ mb: 4 }}>
                    <Toolbar>
                        <Typography variant="h6" sx={{ flexGrow: 1 }}>
                            Token Launchpad
                        </Typography>
                        <Button color="inherit" component={Link} to="/creator">
                            Для Замовника
                        </Button>
                        <Button color="inherit" component={Link} to="/investor">
                            Для Інвестора
                        </Button>
                        <WalletMultiButton />
                    </Toolbar>
                </AppBar>

                {/* Маршрути */}
                <Box sx={{ p: 2 }}>
                    <Routes>
                        <Route path="/creator" element={<CreatorPage />} />
                        <Route path="/investor" element={<InvestorPage />} />
                        <Route path="/" element={<Typography variant="h5" sx={{ textAlign: 'center' }}>Вітаємо! Оберіть роль у меню.</Typography>} />
                    </Routes>
                </Box>
            </WalletContext>
        </BrowserRouter>
    );
};

export default App;

// import { useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
// import './App.css'
//
// function App() {
//   const [count, setCount] = useState(0)
//
//   return (
//     <>
//       <div>
//         <a href="https://vite.dev" target="_blank">
//           <img src={viteLogo} className="logo" alt="Vite logo" />
//         </a>
//         <a href="https://react.dev" target="_blank">
//           <img src={reactLogo} className="logo react" alt="React logo" />
//         </a>
//       </div>
//       <h1>Vite + React</h1>
//       <div className="card">
//         <button onClick={() => setCount((count) => count + 1)}>
//           count is {count}
//         </button>
//         <p>
//           Edit <code>src/App.tsx</code> and save to test HMR
//         </p>
//       </div>
//       <p className="read-the-docs">
//         Click on the Vite and React logos to learn more
//       </p>
//     </>
//   )
// }


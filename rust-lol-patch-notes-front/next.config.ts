import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "am-a.akamaihd.net",
      },
      {
        protocol: "https",
        hostname: "ddragon.leagueoflegends.com",
      },
    ],
  },
};

export default nextConfig;

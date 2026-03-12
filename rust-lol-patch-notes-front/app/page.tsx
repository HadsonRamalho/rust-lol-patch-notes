"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { Input } from "@/components/ui/input";
import { Card } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Search, Loader2 } from "lucide-react";

interface Champion {
  name: string;
  first_image_url: string;
  second_image_url: string;
}

export default function Home() {
  const router = useRouter();
  const [champions, setChampions] = useState<Champion[]>([]);
  const [search, setSearch] = useState("");
  const [language, setLanguage] = useState("pt-br");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchChampions = async () => {
      try {
        const res = await fetch(
          "https://kzg6km3w-3111.brs.devtunnels.ms/list_champions",
        );
        if (!res.ok) throw new Error("Erro ao buscar campeões");

        const data: Champion[] = await res.json();
        setChampions(data);
      } catch (error) {
        console.error("Falha na comunicação com a API RuLo:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchChampions();
  }, []);

  const filteredChampions = champions.filter((champ) =>
    champ.name.toLowerCase().includes(search.toLowerCase()),
  );

  const handleChampionClick = (name: string) => {
    router.push(`/champion/${name}?lang=${language}`);
  };

  return (
    <div className="container mx-auto p-6 min-h-screen flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-8 text-center text-emerald-600">
        RuLo Patch Notes
      </h1>

      <div className="flex flex-col sm:flex-row w-full max-w-2xl items-center gap-4 mb-10">
        <div className="relative w-full">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground h-4 w-4" />
          <Input
            type="text"
            placeholder="Buscar campeão (Ex: Mordekaiser)"
            className="pl-10 w-full"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <Select value={language} onValueChange={setLanguage}>
          <SelectTrigger className="w-full sm:w-[180px]">
            <SelectValue placeholder="Idioma" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="en-us">English</SelectItem>
            <SelectItem value="pt-br">Português (BR)</SelectItem>
            <SelectItem value="de-de">Deutsch</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {loading ? (
        <div className="flex items-center justify-center mt-20">
          <Loader2 className="h-10 w-10 animate-spin text-emerald-600" />
        </div>
      ) : (
        <div className="flex flex-wrap justify-center gap-6">
          {filteredChampions.map((champion) => (
            <Card
              key={champion.name}
              onClick={() => handleChampionClick(champion.name)}
              className="group cursor-pointer overflow-hidden border-2 border-transparent hover:border-emerald-600 transition-all duration-300 w-[120px] bg-secondary/10"
            >
              <div className="relative w-[120px] h-[120px]">
                <Image
                  fill
                  src={champion.first_image_url}
                  alt={champion.name}
                  sizes="120px"
                  className="object-cover group-hover:scale-110 transition-transform duration-300"
                />
                <div className="absolute inset-x-0 bottom-0 bg-black/80 p-2 transform translate-y-full group-hover:translate-y-0 transition-transform duration-300">
                  <p className="text-white text-xs text-center font-medium truncate">
                    {champion.name}
                  </p>
                </div>
              </div>
            </Card>
          ))}

          {filteredChampions.length === 0 && (
            <p className="text-muted-foreground mt-10">
              Nenhum campeão encontrado na base.
            </p>
          )}
        </div>
      )}
    </div>
  );
}

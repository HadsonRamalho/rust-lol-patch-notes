"use client";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Character } from "@/interfaces/character";
import personagens from "@/consts/personagens";
import { Check, X, Search } from "lucide-react";

export default function Home() {
  const router = useRouter();
  const taVisivel = true;

  const todo = [
    // {
    //   id: 1,
    //   titulo: "Adicionar seleção de idioma",
    //   descricao: `Criar um componente de seleção de idioma para permitir que o usuário escolha o idioma desejado.
    //     As opções são: [de-de: Deustch], [en-us: English], [pt-br: Português]
    //     Mas usa o Select do shadcn`,
    //   feito: false,
    // },

    {
      id: 2,
      titulo: "Adicionar pesquisa de personagem pelo nome",
      descricao: `Criar um componente de pesquisa de personagem para permitir que o usuário pesquise personagem por nome.
        Esse filtro não vai buscar no back, só vai filtrar o que o back já tiver retornado (que são todos os personagens).
        Usa o Input do shadcn também`,
      feito: false,
    },
    {
      id: 3,
      titulo: "Representar PatchCard (do back) no front",
      descricao: `Criar um tipo (interface) para representar PatchCard (do back) no front`,
      feito: false,
    },
  ];

  return (
    <div>
      <Select>
        <SelectTrigger className="w-[180px]">
          <SelectValue placeholder="Language" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="en-us">English</SelectItem>
          <SelectItem value="pt-br">Português</SelectItem>
          <SelectItem value="de-de">Deustch</SelectItem>
        </SelectContent>
      </Select>
      <div className="flex w-full max-w-sm items-center gap-2">
        <Input type="Pesquisa" placeholder="Pesquisar" /> <Search />
      </div>

      <h1 className=" p-5 flex items-center justify-center text-2xl">
        League of Legends Champions Patch Notes
      </h1>
      {todo.map((item) => (
        <Card key={item.id}>
          <CardHeader>
            <CardTitle className="flex items-center gap-2"></CardTitle>
            <CardDescription>{item.descricao}</CardDescription>
          </CardHeader>
        </Card>
      ))}
      <div className="flex flex-wrap gap-4">
        {personagens.map((personagem) => (
          <Card
            className="relative max-w-[140px] mx-auto justify-center text-center p-0 hover:scale-104"
            key={personagem.name}
          >
            <div className="relative w-[100px] h-[100px] mx-auto">
              <Image
                fill
                src={`/${personagem.name}.png`}
                alt={personagem.name}
                className="object-cover rounded-md border-2 border-black/65"
              />
              <div className="absolute bottom-0 w-full bg-black/40 rounded-b-md text-white text-xs text-center py-1">
                {personagem.name}
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}

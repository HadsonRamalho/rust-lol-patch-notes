"use client";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
(""); // instalou?
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
// import { Character } from "@/interfaces/character";
import personagens from "@/consts/personagens";
import { Check, X, Search } from "lucide-react";
import { AbilityChanges } from "@//interfaces/character";

export default function Home() {
  const router = useRouter();
  const isVisible = true;

  return (
    <div>
      {/*<Select>
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
      </div>*/}

      <h1 className=" p-5 flex items-center justify-center text-2xl">
        Atualizações de bonecos do LOL
      </h1>
      <div className="justify-items-start flex flex-wrap">
        {personagens.map((personagem) => (
          <Card
            key={personagem.name}
            className={`p-6 m-4 border-1  border-emerald-600 ${isVisible ? "block" : "hidden"}`}
          >
            <CardHeader className="p-5 w-full justify-center text-center ">
              <Image
                width={100}
                height={100}
                className="rounded-full "
                src={`/${personagem.name}.png`}
                alt="o"
              />
              <CardTitle>{personagem.name}</CardTitle>
            </CardHeader>
            <CardContent>
              <Button
                type="button"
                className="bg-red-400 min-w-24 min-h-1/3 "
                onClick={() => {
                  router.push(`about/${personagem.name}`);
                }}
              >
                Ver Atualizações de {personagem.name}
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}

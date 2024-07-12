"use client";
import Link from "next/link";
import {
  ChevronLeft,
  ChevronRight,
  Copy,
  CreditCard,
  File,
  HomeIcon,
  LineChart,
  ListFilter,
  LucideTable2,
  MoreVertical,
  Package,
  Package2,
  PanelLeft,
  Search,
  Settings,
  ShoppingCart,
  Truck,
  Users2,
} from "lucide-react";

import { Badge } from "@/components/ui/badge";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
} from "@/components/ui/pagination";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { TooltipProvider } from "@radix-ui/react-tooltip";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Table {
  id: number;
  name: string;
}

interface TableRows {
  name: string;
  column_type: string;
}

export default function Home() {
  const [files, setFiles] = useState<Table[]>([]);
  const [cols, setCols] = useState<TableRows[]>([]);

  useEffect(() => {
    invoke<Table[]>("get_tables_from_db").then((result) => setFiles(result));
  }, []);

  const fetchTableDetails = (tableName: string) => {
    console.log(tableName);
    invoke<TableRows[]>("get_table_details_from_db", {
      tableName: tableName,
    }).then((result) => setCols(result));
  };

  return (
    <div className="flex flex-row">
      <div className="flex flex-col w-[300px]">
        {files.length > 0 &&
          files.map((table) => (
            <Button
              variant={"ghost"}
              onClick={() => {
                fetchTableDetails(table.name);
              }}
              key={table.name}
              className="flex flex-row items-center justify-start gap-1 text-muted-foreground hover:text-foreground hover:bg-transparent h-fit p-0"
            >
              <LucideTable2 className="w-4 h-4" />
              <p key={table.name}>{table.name}</p>
            </Button>
          ))}
      </div>

      <div className="flex min-h-screen w-full flex-col bg-muted/40">
        <div>
          {cols.map((col) => (
            <div key={col.name}>
              <p>{col.name}</p>
              <p>{col.column_type}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

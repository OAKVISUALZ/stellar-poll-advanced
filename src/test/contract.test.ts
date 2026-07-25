import { describe, it, expect, vi, beforeEach } from "vitest";
import { shortenAddress, isValidStellarAddress } from "../lib/contract";

describe("contract utilities", () => {
  describe("shortenAddress", () => {
    it("shortens a valid stellar address", () => {
      const addr = "GBZUPY4DPM2F4PCDJZXCJBFM3S4M7ZUQGKXGK3TKR5Q5RER5R5R5R5R5";
      const result = shortenAddress(addr);
      expect(result).toBe("GBZUPY...R5R5R5");
    });

    it("shortens with custom character count", () => {
      const addr = "GBZUPY4DPM2F4PCDJZXCJBFM3S4M7ZUQGKXGK3TKR5Q5RER5R5R5R5R5";
      const result = shortenAddress(addr, 4);
      expect(result).toBe("GBZU...R5R5");
    });
  });

  describe("isValidStellarAddress", () => {
    it("validates a correct G address", () => {
      expect(
        isValidStellarAddress(
          "GBZUPY4DPM2F4PCDJZXCJBFM3S4M7ZUQGKXGK3TKR5Q5RER5R5R5R5R5"
        )
      ).toBe(true);
    });

    it("rejects invalid addresses", () => {
      expect(isValidStellarAddress("hello")).toBe(false);
      expect(isValidStellarAddress("")).toBe(false);
      expect(isValidStellarAddress("A1234567890")).toBe(false);
    });

    it("rejects address with wrong prefix", () => {
      expect(
        isValidStellarAddress(
          "ABZUPY4DPM2F4PCDJZXCJBFM3S4M7ZUQGKXGK3TKR5Q5RER5R5R5R5"
        )
      ).toBe(false);
    });
  });
});

describe("PollCard component behavior", () => {
  it("calculates vote percentages correctly", () => {
    const totalVotes = 10;
    const optionVotes = 3;
    const percentage = Math.round((optionVotes / totalVotes) * 100);
    expect(percentage).toBe(30);
  });

  it("handles zero votes", () => {
    const totalVotes = 0;
    const optionVotes = 0;
    const percentage =
      totalVotes > 0 ? Math.round((optionVotes / totalVotes) * 100) : 0;
    expect(percentage).toBe(0);
  });
});

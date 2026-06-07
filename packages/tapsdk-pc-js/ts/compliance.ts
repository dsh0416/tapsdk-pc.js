/**
 * TapTap PC SDK - Compliance functionality
 */

import { native } from './native.js';

/**
 * Compliance API
 */
export class Compliance {
  private readonly _native: ReturnType<typeof native.Compliance.get>;

  private constructor(nativeInstance: ReturnType<typeof native.Compliance.get>) {
    this._native = nativeInstance;
  }

  /**
   * Get the compliance singleton instance.
   *
   * @returns Compliance instance
   * @throws Error if SDK is not initialized
   */
  static get(): Compliance {
    const nativeInstance = native.Compliance.get();
    return new Compliance(nativeInstance);
  }

  /**
   * Ensure the current user has completed real-name verification.
   *
   * The result will be delivered via the ComplianceEnsureRealName event.
   */
  ensureRealName(requestId: number): void {
    this._native.ensureRealName(requestId);
  }

  /**
   * Enable anti-addiction checks.
   *
   * Actions are delivered via the ComplianceActionsEvent event.
   */
  enableAntiAddiction(): void {
    this._native.enableAntiAddiction();
  }

  /**
   * Check whether a payment amount is allowed.
   *
   * @param amount - Payment amount in cents
   */
  checkPaymentLimit(amount: number): { allow: boolean; title: string; description: string } {
    return this._native.checkPaymentLimit(amount);
  }

  /**
   * Submit a successful payment amount.
   *
   * @param amount - Payment amount in cents
   */
  submitPayment(amount: number): void {
    this._native.submitPayment(amount);
  }
}

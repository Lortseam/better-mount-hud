package me.lortseam.bettermounthud.mixin;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.gui.hud.InGameHud;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.entity.LivingEntity;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.*;

@Mixin(InGameHud.class)
public abstract class IngameHudMixin {

    @Shadow @Final private MinecraftClient client;

    @ModifyConstant(method = "renderMountHealth", constant = @Constant(intValue = 39))
    private int bettermounthud$moveMountHealthUp(int original) {
        return original + 10;
    }

    @Redirect(method = "renderStatusBars", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/hud/InGameHud;getHeartCount(Lnet/minecraft/entity/LivingEntity;)I"))
    private int bettermounthud$alwaysRenderFood(InGameHud inGameHud, LivingEntity entity) {
        return 0;
    }

    @Redirect(method = "render", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/network/ClientPlayerEntity;hasJumpingMount()Z"))
    private boolean bettermounthud$switchBar(ClientPlayerEntity player) {
        if (!player.hasJumpingMount()) return false;
        return client.options.keyJump.isPressed() || player.getMountJumpStrength() > 0;
    }

}

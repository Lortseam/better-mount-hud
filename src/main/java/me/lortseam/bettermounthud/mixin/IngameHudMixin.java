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
    private int bettermounthud$moveMountHealthUp(int yOffset) {
        if (client.interactionManager.hasStatusBars()) {
            yOffset += 10;
        }
        return yOffset;
    }

    @Redirect(method = "renderStatusBars", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/hud/InGameHud;getHeartCount(Lnet/minecraft/entity/LivingEntity;)I"))
    private int bettermounthud$alwaysRenderFood(InGameHud inGameHud, LivingEntity entity) {
        return 0;
    }

    @ModifyVariable(method = "renderStatusBars", at = @At(value = "STORE", ordinal = 1), ordinal = 10)
    private int bettermounthud$moveAirUp(int y) {
        if (client.player.hasJumpingMount()) {
            y -= 10;
        }
        return y;
    }

    @Redirect(method = "render", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/network/ClientPlayerEntity;hasJumpingMount()Z"))
    private boolean bettermounthud$switchBar(ClientPlayerEntity player) {
        if (!client.interactionManager.hasExperienceBar()) return player.hasJumpingMount();
        return player.hasJumpingMount() && client.options.keyJump.isPressed() || player.getMountJumpStrength() > 0;
    }

}

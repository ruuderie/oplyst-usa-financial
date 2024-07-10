import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuRadioGroup from '../components/ui/context-menu/ContextMenuRadioGroup.vue';

const meta = {
  title: 'ContextMenuRadioGroup',
  component: ContextMenuRadioGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuRadioGroup>;

export default meta;
type Story = StoryObj<typeof ContextMenuRadioGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
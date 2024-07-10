import type { Meta, StoryObj } from '@storybook/vue3';

import TooltipProvider from '../components/ui/tooltip/TooltipProvider.vue';

const meta = {
  title: 'TooltipProvider',
  component: TooltipProvider,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof TooltipProvider>;

export default meta;
type Story = StoryObj<typeof TooltipProvider>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};